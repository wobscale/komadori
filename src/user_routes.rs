use db;
use rocket::response::Failure;
use rocket::http::Status;

#[post("/")]
pub fn create_user(conn: db::Conn) -> Result<String, Failure> {
	Err(Failure(Status::NotImplemented))
}
