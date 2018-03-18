use std;
use tokio_core::reactor::Handle;
use tokio_core::reactor::Core;
use futures::future::Future;
use rocket::request::{Request, FromRequest};
use rocket::request::Outcome as ReqOutcome;
use rocket::Outcome;
use rocket::response::Responder;
use rocket::http::Status;

pub struct RequestCore {
    core: Core,
}

impl<'r> RequestCore {
    pub fn handle(&self) -> Handle {
        self.core.handle()
    }

    pub fn respond<F, E, Resp>(mut self, f: F) -> Resp
        where F: Future<Item = Resp, Error = E>,
              Resp: Responder<'r>,
              E: std::fmt::Debug,
    {
        self.core.run(f).unwrap()
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for RequestCore {
    type Error = std::io::Error;

    fn from_request(request: &'a Request<'r>) -> ReqOutcome<Self, Self::Error> {
        match Core::new() {
            Err(e) => Outcome::Failure((Status::InternalServerError, e)),
            Ok(core) => {
                Outcome::Success(RequestCore{
                    core: core,
                })
            }
        }
    }
}
