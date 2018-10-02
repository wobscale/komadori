use constant_time_eq;
use rand::Rng;
use rand;
use db;
use rocket;
use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{FromRequest, Request};
use rocket_contrib::json::Json;

use errors::{Error, JsonResult};
use permissions;
use types::UserResp;
use types::CookieUser;

lazy_static! {
    static ref BOOTSTRAP_TOKEN: String = rand::thread_rng()
        .gen_ascii_chars()
        .take(128)
        .collect();
}


pub fn routes() -> Vec<rocket::Route> {
    // Error level so it's always visible
    error!("admin bootstrap token: {}", *BOOTSTRAP_TOKEN);
    routes![bootstrap_admin, list_users]
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
    conn: db::Conn,
    user: CookieUser,
    req: Json<BootstrapAdminReq>,
) -> JsonResult<()> {
    let user = user.0;
    if !constant_time_eq::constant_time_eq(req.token.as_bytes(), (*BOOTSTRAP_TOKEN).as_bytes()) {
        return Err(Error::client_error("invalid bootstrap token".to_string())).into();
    }

    user.add_group(conn, permissions::admin_group().uuid)
        .map_err(|e| {
            Error::server_error(format!("error adding to group: {}", e))
        })
        .map(|_| ()).into()
}

#[derive(Serialize)]
pub struct ListUsersResp {
    users: Vec<UserResp>
}

#[get("/admin/users", format = "application/json")]
pub fn list_users(
    conn: db::Conn,
    _admin: Admin,
) -> JsonResult<ListUsersResp> {
    match db::users::User::list(&conn) {
        Ok(us) => {
            let resp = us.into_iter().map(|u| {
                UserResp::new(u, &conn)
            }).collect::<Result<Vec<_>, _>>();

            match resp {
                Ok(resp) => Ok(ListUsersResp{users: resp}),
                Err(e) => Err(Error::server_error(format!("error listing users: {:?}", e))),
            }
        }
        Err(e) => {
            Err(Error::server_error(format!("error listing users: {:?}", e)))
        }
    }.into()
}

pub struct Admin(CookieUser);

impl<'a, 'r> FromRequest<'a, 'r> for Admin {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> rocket::request::Outcome<Self, ()> {
        let u = match CookieUser::from_request(request) {
            Outcome::Success(u) => u,
            Outcome::Forward(f) => {
                return Outcome::Forward(f);
            }
            Outcome::Failure(f) => {
                return Outcome::Failure(f);
            }
        };
        let db = request.guard::<rocket::State<db::Pool>>()?;
        let db = match db.get() {
            Ok(db) => db,
            Err(e) => {
                error!("error getting db: {}", e);
               return Outcome::Failure((Status::InternalServerError, ()));
            }
        };

        let groups = match u.0.groups(&*db) {
            Err(e) => {
                error!("error getting groups: {:?}", e);
                return Outcome::Failure((Status::InternalServerError, ()));
            }
            Ok(g) => g,
        };

        for group in groups {
            if group.uuid == permissions::admin_group().uuid {
                return Outcome::Success(Admin(u));
            }
        }
        Outcome::Failure((Status::Forbidden, ()))
    }
}
