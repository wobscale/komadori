#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate multi_reactor_drifting;
extern crate hyper;
extern crate hydra_client;
extern crate tokio_core;
extern crate hydra_oauthed_client;
extern crate constant_time_eq;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate github_rs;
#[macro_use]
extern crate lazy_static;
extern crate oauth2;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rand;
extern crate reqwest;
extern crate rocket;
extern crate rocket_contrib;
extern crate url;
extern crate uuid;

extern crate futures;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate chrono;
extern crate fern;
#[macro_use]
extern crate log;

mod request_id;
mod hydra;
mod schema;
mod models;
mod permissions;
mod errors;
pub mod db;
pub mod oauth;
mod admin_routes;
mod user_routes;
mod oauth_routes;
mod github;
mod types;

use rocket::http::Status;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{ContentType, Header, Method};
use diesel::prelude::*;
use std::time::Instant;
use std::io::Cursor;

#[derive(Clone, PartialEq, Debug)]
pub enum Environment{
    Dev,
    Prod,
}

pub struct HydraConfig {
    pub url: String,
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Clone)]
pub struct OauthProviderConfig {
    pub provider: oauth::Provider,
    pub client_id: String,
    pub client_secret: String,
}

pub struct Config {
    pub pool: db::Pool,
    pub oauth: Vec<OauthProviderConfig>,
    pub base_url: String,
    pub hydra: HydraConfig,
    pub environment:  Environment,
}

pub fn rocket(config: Config) -> rocket::Rocket {
    let rkt_conf = match config.environment {
        Environment::Dev => rocket::config::Config::development(),
        Environment::Prod => rocket::config::Config::production(),
    }.unwrap();
    let mut rkt = rocket::custom(rkt_conf, config.environment == Environment::Dev);

    let hydra_builder = {
        hydra::client::ClientBuilder::new(
            &config.hydra.url,
            &config.hydra.client_id,
            &config.hydra.client_secret,
        )
    };

    assert!(config.oauth.len() == 1, "TODO: implement more oauth");
    let oauth_conf = config.oauth[0].clone();
    assert!(oauth_conf.provider == oauth::Provider::Github, "TODO: more oauth");

    {
        let timer = Instant::now();
        db::run_migrations(&config.pool).expect("error running migrations");
        debug!(
            "running migrations took {}",
            (timer.elapsed().as_secs() as f64 + timer.elapsed().subsec_nanos() as f64 * 1e-9)
        );
    }

    permissions::initialize_groups(&config.pool.get().unwrap()).unwrap();

    if rkt.config().environment.is_dev() {
        rkt = rkt.attach(CORS());
    }

    rkt
        .attach(request_id::RequestIDFairing)
        .manage(config.pool)
        .manage(github::OauthConfig::new(oauth_conf.client_id, oauth_conf.client_secret, config.base_url))
        .manage(hydra_builder)
        .mount("/", routes![healthz])
        .mount("/", user_routes::routes())
        .mount("/", admin_routes::routes())
        .mount("/", oauth_routes::routes())
        .mount("/github", github::routes())
}

#[get("/healthz")]
fn healthz(conn: db::Conn) -> Result<String, rocket::response::Failure> {
    conn.execute("SELECT 1")
        .map(|_| "healthy".into())
        .map_err(|e| {
            error!("error executing db healthcheck: {}", e);
            rocket::response::Failure(Status::ServiceUnavailable)
        })
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
