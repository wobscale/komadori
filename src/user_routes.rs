use db;
use types::{CookieUser, PartialUser, UserResp, AuthMetadata, GithubAuthMetadata};
use provider::github::get_github_user;
use oauth;
use diesel;
use diesel::result::Error as DieselErr;
use rocket;
use rocket_contrib::json::Json;
use rocket::http::{Cookie, Cookies};
use errors::Error;

pub fn routes() -> Vec<rocket::Route> {
    routes![create_user, logout_user, get_user]
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    partial_user: PartialUser,
    username: String,
    email: String,
}

#[derive(Debug, Deserialize)]
pub struct GithubLoginRequest {
    code: String,
    state: String,
}

#[derive(Deserialize)]
#[serde(tag = "provider")]
pub enum AuthUserRequest {
    #[serde(rename = "github")]
    Github(GithubLoginRequest),
}

#[get("/user", format = "application/json")]
pub fn get_user(user: CookieUser, conn: db::Conn) -> Result<Json<UserResp>, Json<Error>> {
    let user = user.0;
    UserResp::new(user, &conn)
        .map(|r| {
            Json(r)
        })
        .map_err(|e| Json(e))
}


#[post("/user/create", format = "application/json", data = "<req>")]
pub fn create_user(
    conn: db::Conn,
    req: Json<CreateUserRequest>,
    mut cookies: Cookies,
) -> Json<Result<UserResp, Error>> {
    if req.username.len() == 0 {
        return Json(Err(Error::client_error("Name cannot be blank".to_string())));
    }
    if req.email.len() == 0 {
        return Json(Err(Error::client_error(
            "Email cannot be blank".to_string(),
        )));
    }

    let gh = match get_github_user(&req.partial_user.access_token) {
        Ok(u) => u,
        Err(e) => return Json(Err(e)),
    };

    let create_res = match req.partial_user.provider {
        oauth::Provider::Github => db::users::NewUser{
            username: &req.username,
            email: &req.email,
        }.insert_github(&*conn, db::users::NewGithubAccount{
            id: req.partial_user.provider_id,
            access_token: &req.partial_user.access_token,
        })
    };

    match create_res {
        Err(e) => {
            match e {
                DieselErr::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, e) => {
                    match e.constraint_name() {
                        Some("users_username_key") => {
                            Json(Err(Error::client_error(format!("Could not create account; username '{}' already exists.", req.username))))
                        }
                        Some("github_accounts_pkey") => {
                            Json(Err(Error::client_error(format!("Could not create account; Github account with id {} already associated with a user.", req.partial_user.provider_id))))
                        }
                        _ => {
                            error!("uniqueness violation case missed: {:?}: {:?}, {:?}", e, e.table_name(), e.column_name());
                            Json(Err(Error::client_error("An unknown uniqueness violation happened, sorry :(".to_string())))
                        }
                    }
                },
                _ => {
                    error!("error creating user account: {}", e);
                    Json(Err(Error::server_error("Could not create account: please contact the administrator if this persists".to_string())))
                }
            }
        }
        Ok(newuser) => {
            cookies.add_private(Cookie::new(
                "user_uuid".to_owned(),
                newuser.uuid.simple().to_string(),
            ));
            cookies.remove_private(Cookie::named("oauth_token"));
            Json(Ok(UserResp{
                username: newuser.username,
                email: newuser.email,
                role: None,
                uuid: newuser.uuid.simple().to_string(),
                groups: vec![],
                auth_metadata: AuthMetadata{
                    github: Some(Ok(GithubAuthMetadata{
                        username: gh.login,
                    })),
                },
            }))
        }
    }
}

#[derive(Serialize)]
pub struct UserLogoutResponse {}

#[get("/user/logout")]
pub fn logout_user(mut cookies: Cookies) -> Json<UserLogoutResponse> {
    cookies.remove_private(Cookie::named("oauth_token"));
    cookies.remove_private(Cookie::named("user_uuid"));
    Json(UserLogoutResponse {})
}
