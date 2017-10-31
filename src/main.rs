#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rand;
extern crate rocket;
use rocket::http::{Cookie, Cookies};
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate oauth2;

extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod db;
mod user_routes;
mod github;

use rocket::http::Status;
use rocket::response::NamedFile;
use std::path::{Path, PathBuf};
use diesel::prelude::*;
use std::env;

#[macro_use]
extern crate log;
extern crate env_logger;

#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/index.html")).ok()
}

#[get("/healthz")]
fn test(conn: db::Conn) -> Result<String, rocket::response::Failure> {
    conn.execute("SELECT 1").map(|_| "healthy".into()).map_err(
        |e| {
            error!("error executing db healthcheck: {}", e);
            rocket::response::Failure(Status::ServiceUnavailable)
        },
    )
}

#[get("/<file..>", rank = 3)]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn main() {
    env_logger::init();
    let rkt = rocket::ignite();

    let base_url = if rkt.config().environment.is_dev() {
        format!("http://127.0.0.1:{}", rkt.config().port)
    } else {
        env::var("BASE_URL").expect("Must set BASE_URL")
    };

    let pool = {
        let uri = &env::var("DATABASE_URL").expect("DATABASE_URL environment variable must be set");
        db::db_pool(uri).expect("error connecting to database")
    };

    let github_oauth_config = {
        let client_id = env::var("GITHUB_CLIENT_ID").expect("GITHUB_CLIENT_ID must be set");
        let client_secret = env::var("GITHUB_SECRET_KEY").expect("GITHUB_SECRET_KEY must be set");
        let github_base_url = format!("{}/{}", base_url, "github");
        github::OauthConfig::new(client_id, client_secret, github_base_url)
    };

    db::run_migrations(&pool).expect("error running migrations");

    rkt.manage(pool)
        .manage(github_oauth_config)
        .mount("/", routes![index, test, files])
        .mount("/user", user_routes::routes())
        .mount("/github", github::routes())
        .launch();
}
