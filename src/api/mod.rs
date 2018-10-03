use rocket;
use types::OauthUser;
use errors::{Error, JsonResult};
use uuid::Uuid;
use db;

pub fn routes() -> Vec<rocket::Route> {
    routes![group_contains_user]
}

#[derive(Debug, Serialize)]
pub struct MembershipResp {
    member: bool
}

#[get("/group/<guid>/membership", format = "application/json")]
pub fn group_contains_user(user: OauthUser, conn: db::Conn, guid: String) -> JsonResult<MembershipResp> {
    let guid = match Uuid::parse_str(&guid).map_err(|_| Error::client_error("invalid guid format".to_owned())) {
        Err(e) => {
            return Err(e).into();
        }
        Ok(g) => g,
    };
    if !user.scopes.contains(&"group:check_membership".to_owned()) {
        return Err(Error::client_error("permission denied; insufficient scopes".to_owned())).into();
    }

    let mut groups = match user.user.groups(&conn) {
        Err(e) => {
            warn!("err getting user groups: {}", e);
            return Err(Error::server_error("error getting groups".to_owned())).into();
        },
        Ok(g) => {
            g
        },
    };

    groups.retain(|g| g.uuid == guid);
    Ok(MembershipResp{
        member: groups.len() > 0
    }).into()
}
