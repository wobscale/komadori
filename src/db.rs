use diesel::PgConnection;
use r2d2;
use r2d2_diesel::ConnectionManager;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
use std::ops::Deref;

embed_migrations!();

pub fn db_pool(uri: &str) -> Result<Pool, r2d2::InitializationError> {
    let config = r2d2::Config::default();
    let manager = ConnectionManager::new(uri);
    r2d2::Pool::new(config, manager)
}

pub fn run_migrations(pool: &Pool) -> Result<(), String> {
    let db_conn = pool.get().expect("could not get connection from pool");
    let mut migration_out = Vec::new();
    embedded_migrations::run_with_output(&*db_conn, &mut migration_out)
        .expect("error running migrations");
    if migration_out.len() > 0 {
        info!(
            "migrations run: \n{}",
            String::from_utf8_lossy(&migration_out)
        );
    };
    Ok(())
}

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

// Sourced partially from https://rocket.rs/guide/state/#databases
pub struct Conn(r2d2::PooledConnection<ConnectionManager<PgConnection>>);
/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for Conn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Conn, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(Conn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for Conn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
