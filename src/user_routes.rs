use db;
use db::users::User as DBUser;
use types::{CookieUser, PartialUser};
use oauth;
use diesel;
use diesel::result::Error as DieselErr;
use rocket;
use rocket::State;
use rocket_contrib::json::Json;
use rocket::http::{Cookie, Cookies};
use github;
use errors::Error;

pub fn routes() -> Vec<rocket::Route> {
    routes![create_user, logout_user, auth_user, get_user]
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    partial_user: PartialUser,
    username: String,
    email: String,
}

#[derive(Debug, Serialize)]
pub struct GroupResp {
    pub uuid: String,
    pub name: String,
    pub description: String,
    pub public: bool,
}

impl<'a> From<&'a db::groups::Group> for GroupResp {
    fn from(g: &'a db::groups::Group) -> Self {
        GroupResp {
            uuid: g.uuid.simple().to_string(),
            name: g.name.clone(),
            description: g.description.clone(),
            public: g.public,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct UserResp {
    pub uuid: String,
    pub username: String,
    pub role: Option<String>,
    pub email: String,

    pub groups: Vec<GroupResp>,

    pub auth_metadata: AuthMetadata,
}

#[derive(Debug, Serialize)]
pub struct AuthMetadata {
    pub github: Option<Result<GithubAuthMetadata, String>>,
}

#[derive(Debug, Serialize)]
pub struct GithubAuthMetadata {
    username: String,
}

impl UserResp {
    pub fn new(user: DBUser, conn: &db::Conn) -> Result<UserResp, Error> {

        let groups = user.groups(conn)
            .map_err(|e| {
                Error::server_error(format!("error getting groups: {}", e))
            })?;

        // TODO: more oauth providers will break this
        let github_account = {
            user.github_account(conn)
                .map_err(|e| {
                    Error::server_error(format!("error getting accounts: {}", e))
                })
            .and_then(|c| {
                github::get_github_user(&c.access_token)
            }).map(|gh| GithubAuthMetadata{
                username: gh.login
            })
            .map_err(|e| format!("{:?}", e))
        };

        Ok(UserResp {
            uuid: user.uuid.simple().to_string(),
            username: user.username,
            role: user.role,
            email: user.email,
            groups: groups.iter().map(|g| g.into()).collect(),
            auth_metadata: AuthMetadata{
                github: Some(github_account),
            },
        })
    }
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

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum AuthUserResp {
    UserResp(UserResp),
    PartialUser(PartialUser),
}

#[post("/user/auth", format = "application/json", data = "<req>")]
pub fn auth_user(
    conn: db::Conn,
    req: Json<AuthUserRequest>,
    github_oauth: State<github::OauthConfig>,
    mut cookies: Cookies,
) -> Json<Result<AuthUserResp, Error>> {
    match req.0 {
        AuthUserRequest::Github(g) => {
            // We got github oauth tokens, exchange it for an access code
            let token = match github_oauth.config().exchange_code(g.code.clone()) {
                Ok(t) => t,
                Err(e) => {
                    error!("github exchange code error: {}", e);
                    return Json(Err(Error::client_error(
                        "could not exchange code".to_string(),
                    )));
                }
            };

            let user = match github::get_github_user(&token.access_token) {
                Ok(u) => u,
                Err(e) => return Json(Err(e)),
            };

            let pu = PartialUser {
                provider: oauth::Provider::Github,
                provider_id: user.id,
                provider_name: user.login,
                access_token: token.access_token,
            };

            // Now either this github account id could have an associated user, or not. If it does,
            // return it, if not, assume this is a partial user.
            match DBUser::from_oauth_provider(&conn, &pu.provider, &pu.provider_id) {
                Ok(u) => {
                    cookies.add_private(Cookie::new(
                        "user_uuid".to_owned(),
                        u.uuid.simple().to_string(),
                    ));
                    let ru = match UserResp::new(u, &conn) {
                        Err(e) => {
                            return Json(Err(e));
                        }
                        Ok(ru) => ru,
                    };
                    Json(Ok(AuthUserResp::UserResp(ru)))
                }
                Err(e) => {
                    debug!("error getting user from partial user; assuming user doesn't exist: {:?}", e);
                    // TODO: better error handling for client vs server errs
                    Json(Ok(AuthUserResp::PartialUser(pu)))
                }
            }
        }
    }
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

    let gh = match github::get_github_user(&req.partial_user.access_token) {
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
