use db;
use types::{User, CookieUser, PartialUser, UserResp, AuthMetadata, GithubAuthMetadata,AuthUserResp};
use provider::github::get_github_user;
use oauth;
use diesel;
use rocket::State;
use diesel::result::Error as DieselErr;
use rocket;
use rocket_contrib::json::Json;
use rocket::http::{Cookie, Cookies};
use errors::{JsonResult, Error};
use provider::{ProviderSet, ProviderAuthRequest};

pub fn routes() -> Vec<rocket::Route> {
    routes![auth_user, create_user, logout_user, get_user]
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    partial_user: PartialUser,
    username: String,
    email: String,
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
) -> JsonResult<UserResp> {
    if req.username.len() == 0 {
        return Err(Error::client_error("Name cannot be blank".to_string())).into();
    }
    if req.email.len() == 0 {
        return Err(Error::client_error(
            "Email cannot be blank".to_string(),
        )).into();
    }

    let mut auth_meta = AuthMetadata{
        github: None,
    };

    let create_res = match req.partial_user.provider {
        oauth::Provider::Github => {
            let gh = match get_github_user(&req.partial_user.access_token) {
                Ok(u) => u,
                Err(e) => return Err(e).into(),
            };
            auth_meta.github = Some(Ok(GithubAuthMetadata{
                username: gh.login,
            }));
            db::users::NewUser{
                username: &req.username,
                email: &req.email,
            }.insert_github(&*conn, db::users::NewGithubAccount{
                id: req.partial_user.provider_id,
                access_token: &req.partial_user.access_token,
            })
        }
        oauth::Provider::Local => db::users::NewUser{
            username: &req.username,
            email: &req.email,
        }.insert_local(&*conn, db::users::NewLocalAccount{
            id: req.partial_user.provider_id,
        })
    };

    match create_res {
        Err(e) => {
            match e {
                DieselErr::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, e) => {
                    match e.constraint_name() {
                        Some("users_username_key") => {
                            Err(Error::client_error(format!("Could not create account; username '{}' already exists.", req.username)))
                        }
                        Some("github_accounts_pkey") => {
                            Err(Error::client_error(format!("Could not create account; Github account with id {} already associated with a user.", req.partial_user.provider_id)))
                        }
                        _ => {
                            error!("uniqueness violation case missed: {:?}: {:?}, {:?}", e, e.table_name(), e.column_name());
                            Err(Error::client_error("An unknown uniqueness violation happened, sorry :(".to_string()))
                        }
                    }
                },
                _ => {
                    error!("error creating user account: {}", e);
                    Err(Error::server_error("Could not create account: please contact the administrator if this persists".to_string()))
                }
            }
        }
        Ok(newuser) => {
            cookies.add_private(Cookie::new(
                "user_uuid".to_owned(),
                newuser.uuid.simple().to_string(),
            ));
            cookies.remove_private(Cookie::named("oauth_token"));
            Ok(UserResp{
                username: newuser.username,
                email: newuser.email,
                role: None,
                uuid: newuser.uuid.simple().to_string(),
                groups: vec![],
                auth_metadata: auth_meta,
            })
        }
    }.into()
}

#[derive(Serialize)]
pub struct UserLogoutResponse {}

#[get("/user/logout")]
pub fn logout_user(mut cookies: Cookies) -> Json<UserLogoutResponse> {
    cookies.remove_private(Cookie::named("oauth_token"));
    cookies.remove_private(Cookie::named("user_uuid"));
    Json(UserLogoutResponse {})
}

#[post("/user/auth", format = "application/json", data = "<req>")]
pub fn auth_user(mut cookies: Cookies, conn: db::Conn, providers: State<ProviderSet>, req: Json<ProviderAuthRequest>) -> Json<Result<AuthUserResp, Error>> {
    let pu = match providers.partial_user(&*req) {
        Ok(pu) => pu,
        Err(e) => { return Json(Err(e)) },
    };

    let user = match User::from_oauth_provider(&conn, &pu.provider, &pu.provider_id) {
        Ok(u) => {
            cookies.add_private(Cookie::new(
                "user_uuid".to_owned(),
                u.uuid.simple().to_string(),
            ));
            u
        }
        Err(e) => {
            debug!("error getting user from partial user; assuming user doesn't exist: {:?}", e);
            // TODO: better error handling for client vs server errs
            return Json(Ok(AuthUserResp::PartialUser(pu)));
        }
    };
    match UserResp::new(user, &conn) {
        Err(e) => Json(Err(e)),
        Ok(u) => Json(Ok(AuthUserResp::UserResp(u))),
    }
}
