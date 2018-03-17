extern crate rocket;

use rocket::request::Request;
use rocket::http::Status;
use rocket::response::Response;
use rocket::request::FromRequest;
use rocket::request::Outcome as ReqOutcome;
use rocket::Outcome;
use rand::{thread_rng, Rng};
use std::collections::hash_map;
use std::sync::Mutex;
use std::ops::Deref;

lazy_static!{
    static ref REQUEST_IDS: Mutex<hash_map::HashMap<usize, u64, hash_map::RandomState>> = Mutex::new(hash_map::HashMap::new());
}

pub struct RequestIDFairing;

impl<'r> rocket::fairing::Fairing for RequestIDFairing {
    fn info(&self) -> rocket::fairing::Info {
        rocket::fairing::Info {
            kind: rocket::fairing::Kind::Request | rocket::fairing::Kind::Response,
            name: "request id",
        }
    }

    // When we get a request, create an id and associate it with it by laying it out in memory near
    // the request... and then don't free the memory so we can find the id later
    fn on_request(&self, request: &mut Request, _: &rocket::Data) { 
        REQUEST_IDS.lock().unwrap().insert(request as *const Request as usize, thread_rng().gen());
    }
    fn on_response(&self, request: &Request, _: &mut Response) {
        REQUEST_IDS.lock().unwrap().remove(&(request as *const Request as usize));
    }
}

pub struct RequestID {
    id: u64
}

impl Deref for RequestID {
    type Target = u64;

    fn deref(&self) -> &u64{
        &self.id
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for RequestID {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> ReqOutcome<Self, Self::Error> {
        match REQUEST_IDS.lock().unwrap().get(&(request as *const Request as usize)) {
            Some(id) => Outcome::Success(RequestID {id: id.clone()}),
            None => {
                error!("unable to get request id: did you forget to attach the fairing?");
                Outcome::Failure((Status::InternalServerError, ()))
            }
        }
    }
}
