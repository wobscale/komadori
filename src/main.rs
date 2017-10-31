#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate r2d2;
extern crate r2d2_diesel;

extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod db;
mod user_routes;

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
    let pool = {
        let uri = &env::var("DATABASE_URL").expect("DATABASE_URL environment variable must be set");
        db::db_pool(uri).expect("error connecting to database")
    };

    db::run_migrations(&pool).expect("error running migrations");

    rocket::ignite()
        .manage(pool)
        .mount("/", routes![index, test, files])
        .launch();
}
