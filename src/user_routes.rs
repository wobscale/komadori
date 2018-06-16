use db;
use db::users::User as DBUser;
use oauth;
use diesel;
use diesel::result::Error as DieselErr;
use rocket;
use rocket::State;
use rocket_contrib::json::Json;
use uuid::Uuid;
use rocket::http::Status;
use rocket::Outcome;
use rocket::request::{FromRequest, Request};
use rocket::http::{Cookie, Cookies};
use github_rs::client::Executor;
use github_rs::client::Github;
use github;
use errors::Error;

pub fn routes() -> Vec<rocket::Route> {
    routes![create_user, logout_user, auth_user, get_user]
}

pub type User = DBUser;

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> rocket::request::Outcome<Self, ()> {
        let uuid_ = {
            // ensure cookies is dropped before the PartialUser::from_request so we don't error out
            // on too many cookies
            let mut cookies = request.cookies();
            match cookies.get_private("user_uuid") {
                Some(uuid) => {
                    debug!("got user_uuid from cookie: {}", uuid);
                    match Uuid::parse_str(&uuid.value()) {
                        Ok(u) => Some(u),
                        Err(e) => {
                            error!("could not decode user's uuid: {}", e);
                            return Outcome::Failure((Status::InternalServerError, ()));
                        }
                    }
                }
                None => {
                    debug!("cookie had no user_uuid");
                    None
                }
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

        let uuid_ = match uuid_ {
            Some(u) => u,
            None => {
                // If we have a github oauth login thing going, let's try that
                debug!("attempting to get uuid from partial-user");
                match PartialUser::from_request(request) {
                    Outcome::Success(pu) => {
                        match DBUser::from_oauth_provider(&*db, &pu.provider, &pu.provider_id) {
                            Ok(u) => {
                                // We should also save this user in the cookie to avoid the
                                // from_partial_user next time
                                // TODO: this is absolutely the wrong place code-organization-wise
                                // to do this
                                {
                                    let mut cookies = request.cookies();
                                    cookies.add_private(Cookie::new(
                                        "user_uuid".to_owned(),
                                        u.uuid.simple().to_string(),
                                    ));
                                }
                                u.uuid
                            }
                            Err(e) => {
                                error!("could not create partial user from partial user: {:?}", e);
                                return Outcome::Failure((Status::InternalServerError, ()));
                            }
                        }
                    }
                    Outcome::Forward(()) => return Outcome::Forward(()),
                    Outcome::Failure(e) => {
                        return Outcome::Failure(e);
                    }
                }
            }
        };
        match User::from_uuid(&*db, uuid_) {
            Err(db::users::GetUserError::NoSuchUser) => {
                Outcome::Failure((Status::NotFound, ()))
            },
            Err(e) => {
                error!("error using uuid to get user: {:?}", e);
                Outcome::Failure((Status::InternalServerError, ()))
            }
            Ok(u) => Outcome::Success(u),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartialUser {
    provider: oauth::Provider,
    provider_id: i32,
    provider_name: String,
    access_token: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for PartialUser {
    type Error = ();


    fn from_request(request: &'a Request<'r>) -> rocket::request::Outcome<Self, ()> {
        let token = match oauth::SerializableToken::from_request(request) {
            Outcome::Success(token) => token,
            Outcome::Forward(()) => {
                return Outcome::Forward(());
            }
            Outcome::Failure(e) => {
                return Outcome::Failure(e);
            }
        };
        // Let's make sure the token is valid..
        let (uid, name) = match token.provider {
            oauth::Provider::Github => {
                let client = match Github::new(&token.token.access_token) {
                    Ok(c) => c,
                    Err(e) => {
                        error!("could not create client: {}", e);
                        return Outcome::Failure((Status::InternalServerError, ()));
                    }
                };


                #[derive(Deserialize)]
                struct GithubUser {
                    _email: Option<String>,
                    _name: Option<String>,
                    login: String,
                    id: i32,
                    _avatar_url: Option<String>,
                }


                let user = match client.get().user().execute::<GithubUser>() {
                    Err(e) => {
                        error!("could not get github user: {}", e);
                        return Outcome::Failure((Status::InternalServerError, ()));
                    }
                    Ok((_, _, None)) => {
                        return Outcome::Failure((Status::InternalServerError, ()));
                    }
                    Ok((_, _, Some(u))) => u,
                };
                (user.id, user.login)
            }
        };
        Outcome::Success(PartialUser {
            provider: token.provider,
            provider_id: uid,
            provider_name: name,
            access_token: token.token.access_token,
        })
    }
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
}

impl UserResp {
    pub fn new(user: DBUser, conn: db::Conn) -> Result<UserResp, Error> {

        let groups = user.groups(conn)
            .map_err(|e| {
                Error::server_error(format!("error getting groups: {}", e))
            })?;

        Ok(UserResp {
            uuid: user.uuid.simple().to_string(),
            username: user.username,
            role: user.role,
            email: user.email,
            groups: groups.iter().map(|g| g.into()).collect(),
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

            let client = match Github::new(&token.access_token) {
                Ok(c) => c,
                Err(e) => {
                    return Json(Err(Error::server_error(format!(
                        "could not create github client: {}",
                        e
                    ))));
                }
            };

            // Now use the access token to get this user's github info. id is the important thing.
            #[derive(Deserialize)]
            struct GithubUser {
                _email: Option<String>,
                _name: Option<String>,
                login: String,
                id: i32,
                _avatar_url: Option<String>,
            }

            let user = match client.get().user().execute::<GithubUser>() {
                Err(e) => {
                    error!("could not get github user: {}", e);
                    return Json(Err(Error::client_error(
                        "could not get github user with token".to_string(),
                    )));
                }
                Ok((_, _, None)) => {
                    return Json(Err(Error::server_error(
                        "Github returned success, but with no user??".to_string(),
                    )));
                }
                Ok((_, _, Some(u))) => u,
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
                    let ru = match UserResp::new(u, conn) {
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
pub fn get_user(user: User, conn: db::Conn) -> Result<Json<UserResp>, Json<Error>> {
    UserResp::new(user, conn)
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
