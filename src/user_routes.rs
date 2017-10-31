use db;
use rocket;
use rocket::response::Failure;
use rocket::http::Status;
use rocket_contrib::Json;

pub fn routes() -> Vec<rocket::Route> {
    routes![create_user]
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    username: String,
    display_name: Option<String>,
    email: Option<String>,

    github_id: i64,
}

#[post("/", format = "application/json", data = "<user>")]
pub fn create_user(conn: db::Conn, user: Json<CreateUserRequest>) -> Result<String, Failure> {
    Err(Failure(Status::NotImplemented))
}
