use rocket;
use rocket_contrib::json::Json;
use types::OauthUser;
use errors::Error;
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
pub fn group_contains_user(user: OauthUser, conn: db::Conn, guid: String) -> Result<Json<MembershipResp>, Json<Error>> {
    let guid = Uuid::parse_str(&guid).map_err(|_| Json(Error::client_error("invalid guid format".to_owned())))?;
    if !user.scopes.contains(&"group:check_membership".to_owned()) {
        return Err(Json(Error::client_error("permission denied; insufficient scopes".to_owned())));
    }

    let mut groups = match user.user.groups(&conn) {
        Err(e) => {
            warn!("err getting user groups: {}", e);
            return Err(Json(Error::server_error("error getting groups".to_owned())));
        },
        Ok(g) => {
            g
        },
    };

    groups.retain(|g| g.uuid == guid);
    Ok(Json(MembershipResp{
        member: groups.len() > 0
    }))
}
