use constant_time_eq;
use hydra_client;
use tokio_rocket;
use futures;
use futures::Future;
use rand::Rng;
use rand;
use rocket::State;
use rocket;
use rocket_contrib::json::Json;

use errors::Error;
use hydra;
use permissions;
use user_routes::User;

lazy_static! {
    static ref BOOTSTRAP_TOKEN: String = rand::thread_rng()
        .gen_ascii_chars()
        .take(128)
        .collect();
}


pub fn routes() -> Vec<rocket::Route> {
    // Error level so it's always visible
    error!("admin bootstrap token: {}", *BOOTSTRAP_TOKEN);
    routes![bootstrap_admin]
}

#[derive(Deserialize)]
pub struct BootstrapAdminReq {
    token: String,
}

// This is the only route here which can be accessed by a non-administrator; it allows someone with
// access to the logs of this app (aka an admin) to upgrade their regular user account to an
// administrator account.
// This is used to bootstrap the first administrator
// TODO: conceivably this should be only usable if the admin group is empty
#[post("/admin/bootstrap", format = "application/json", data = "<req>")]
pub fn bootstrap_admin(
    user: User,
    hydra: State<hydra::client::ClientBuilder>,
    handle: tokio_rocket::Handle,
    req: Json<BootstrapAdminReq>,
) -> tokio_rocket::Future<Json<()>, Json<Error>> {
    if !constant_time_eq::constant_time_eq(req.token.as_bytes(), (*BOOTSTRAP_TOKEN).as_bytes()) {
        return tokio_rocket::Future(Box::new(futures::future::err(Json(Error::client_error("invalid bootstrap token".to_string())))));
    }
    let client = hydra.build(&(handle.into()));
    let uuidstr = user.uuid.simple().to_string();
    tokio_rocket::Future(Box::new(client.client().warden_api().add_members_to_group(permissions::ADMIN_GROUP, hydra_client::models::GroupMembers::new().with_members(vec![uuidstr]))
        .map(|o| Json(()))
        .map_err(|e| {
            Json(Error::server_error(format!("error: {:?}", e)))
        })))
}
