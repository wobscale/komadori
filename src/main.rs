#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate github_rs;
extern crate oauth2;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rand;
extern crate reqwest;
extern crate rocket;
extern crate rocket_contrib;
extern crate url;
extern crate uuid;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod hydra;
mod schema;
mod models;
mod permissions;
mod errors;
mod db;
mod oauth;
mod user_routes;
mod oauth_routes;
mod github;

use rocket::http::Status;
use rocket::response::NamedFile;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{ContentType, Header, Method};
use std::path::{Path, PathBuf};
use diesel::prelude::*;
use std::env;
use std::time::Instant;
use rocket_contrib::Template;
use std::io::Cursor;

extern crate chrono;
extern crate fern;
#[macro_use]
extern crate log;

#[get("/healthz")]
fn healthz(conn: db::Conn) -> Result<String, rocket::response::Failure> {
    conn.execute("SELECT 1")
        .map(|_| "healthy".into())
        .map_err(|e| {
            error!("error executing db healthcheck: {}", e);
            rocket::response::Failure(Status::ServiceUnavailable)
        })
}

#[get("/<file..>", rank = 3)]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn main() {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}]\t[{}]\t {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LogLevelFilter::Warn)
        .level_for("rocket", log::LogLevelFilter::Info)
        .level_for("komadori", log::LogLevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()
        .unwrap();
    let mut rkt = rocket::ignite();

    let base_url = if rkt.config().environment.is_dev() {
        env::var("BASE_URL").unwrap_or(format!("http://127.0.0.1:{}", rkt.config().port))
    } else {
        env::var("BASE_URL").expect("Must set BASE_URL")
    };

    if rkt.config().environment.is_dev() {
        rkt = rkt.attach(CORS());
    }

    let pool = {
        let uri = &env::var("DATABASE_URL").expect("DATABASE_URL environment variable must be set");
        db::db_pool(uri).expect("error connecting to database")
    };

    let hydra = {
        hydra::client::Client::new(
            &env::var("HYDRA_URL").expect("Must set HYDRA_URL"),
            &env::var("HYDRA_CLIENT_ID").expect("Must set HYDRA_CLIENT_ID"),
            &env::var("HYDRA_CLIENT_SECRET").expect("Must set HYDRA_CLIENT_SECRET"),
        )
    };
    permissions::initialize_groups(&hydra).expect("could not initialize groups");

    let github_oauth_config = {
        let client_id = env::var("GITHUB_CLIENT_ID").expect("GITHUB_CLIENT_ID must be set");
        let client_secret = env::var("GITHUB_SECRET_KEY").expect("GITHUB_SECRET_KEY must be set");
        github::OauthConfig::new(client_id, client_secret, base_url)
    };

    {
        let timer = Instant::now();
        db::run_migrations(&pool).expect("error running migrations");
        debug!(
            "running migrations took {}",
            (timer.elapsed().as_secs() as f64 + timer.elapsed().subsec_nanos() as f64 * 1e-9)
        );
    }

    rkt.attach(Template::fairing())
        .manage(pool)
        .manage(github_oauth_config)
        .manage(hydra)
        .mount("/", routes![healthz, files])
        .mount("/", user_routes::routes())
        .mount("/", oauth_routes::routes())
        .mount("/github", github::routes())
        .launch();
}

pub struct CORS();

impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to requests",
            kind: Kind::Response,
        }
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        response.set_header(Header::new(
            "Access-Control-Allow-Origin",
            "http://localhost:3000",
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));

        if request.method() == Method::Options {
            response.set_status(Status::Ok);
            response.set_header(ContentType::Plain);
            response.set_sized_body(Cursor::new(""));
        }
    }
}
