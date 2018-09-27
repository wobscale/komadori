use rocket::{Request, Response};
use rocket;
use serde::Serialize;
use rocket::http::Status;
use rocket_contrib::json::Json;
use std::io::Cursor;
use serde_json;

#[derive(Serialize, Debug)]
pub struct Error {
    #[serde(with = "StatusDef")] status: Status,
    message: String,
}

impl Error {
    pub fn new(status: Status, message: String) -> Self {
        Self {
            status: status,
            message: message,
        }
    }
    pub fn client_error(message: String) -> Self {
        Self::new(Status::BadRequest, message)
    }
    pub fn server_error(message: String) -> Self {
        Self::new(Status::InternalServerError, message)
    }
}

impl<'a> rocket::response::Responder<'a> for Error {
    fn respond_to(self, _: &Request) -> Result<Response<'a>, Status> {
        let json_body = serde_json::to_string(&self).map_err(|e| {
            error!("could not serialize json: {}", e);
            Status::InternalServerError
        })?;
        Response::build()
            .status(self.status)
            .header(rocket::http::ContentType::JSON)
            .sized_body(Cursor::new(json_body))
            .ok()
    }
}

// Remote serialize impl for status
#[derive(Serialize)]
#[serde(remote = "Status")]
pub struct StatusDef {
    pub code: u16,
    pub reason: &'static str,
}



pub enum JsonResult<T> {
    Ok(T),
    Err(Error),
}

impl<T> From<Result<T, Error>> for JsonResult<T> {
    fn from(other: Result<T, Error>) -> Self {
        match other {
            Ok(t) => JsonResult::Ok(t),
            Err(e) => JsonResult::Err(e),
        }
    }
}

impl<'r, T> rocket::response::Responder<'r> for JsonResult<T>
    where T: Serialize
{
        fn respond_to(self, req: &Request) -> rocket::response::Result<'r> {
            match self {
                JsonResult::Ok(t) => {
                    Json(t).respond_to(req)
                },
                JsonResult::Err(e) => {
                    e.respond_to(req)
                }
            }
        }
}

