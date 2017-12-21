use std;
use db;
use oauth;
use diesel;
use diesel::Connection;
use diesel::result::Error as DieselErr;
use rocket;
use rocket::State;
use rocket_contrib::json::Json;
use std::time::Instant;
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

#[derive(Debug, Clone, Queryable)]
pub struct User {
    _id: i32,
    pub uuid: Uuid,
    pub username: String,
    pub role: Option<String>,
    pub email: String,
    _created: std::time::SystemTime,
    _updated: std::time::SystemTime,
}

#[derive(Debug, Clone, Queryable)]
pub struct GithubUser {
    pub _id: i32,
    user_id: i32,
    access_token: String,
}

#[derive(Debug)]
enum GetUserError {
    DbError(diesel::result::Error),
    NoSuchUser,
}

impl User {
    fn from_uuid(conn: &diesel::PgConnection, uuid_: Uuid) -> Result<Self, GetUserError> {
        use diesel::prelude::*;
        use schema::users::dsl::*;
        match users.filter(uuid.eq(uuid_)).limit(1).load::<User>(conn) {
            Ok(u) => match u.first() {
                Some(u) => Ok(u.clone()),
                None => {
                    error!("error getting user {}", uuid_);
                    Err(GetUserError::NoSuchUser)
                }
            },
            Err(e) => {
                error!("error getting user {}", uuid_);
                Err(GetUserError::DbError(e))
            }
        }
    }
    fn from_partial_user(
        conn: &diesel::PgConnection,
        pu: &PartialUser,
    ) -> Result<Self, GetUserError> {
        // Compile-check that we can assume github's the only provider
        match pu.provider {
            oauth::Provider::Github => (),
        };

        use diesel::prelude::*;
        use schema::github_accounts;
        use schema::users::dsl::*;
        match {
            let timer = Instant::now();
            let res = users
                .inner_join(github_accounts::table)
                .select(users::all_columns())
                .filter(github_accounts::id.eq(pu.provider_id))
                .limit(1)
                .load::<User>(conn);
            debug!(
                "Partial user to user query took {}",
                (timer.elapsed().as_secs() as f64 + timer.elapsed().subsec_nanos() as f64 * 1e-9)
            );
            res
        } {
            Ok(u) => match u.first() {
                Some(u) => Ok(u.clone()),
                None => Err(GetUserError::NoSuchUser),
            },
            Err(e) => Err(GetUserError::DbError(e)),
        }
    }
}

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
                        match User::from_partial_user(&*db, &pu) {
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
pub struct UserResp {
    pub uuid: String,
    pub username: String,
    pub role: Option<String>,
    pub email: String,
}

impl UserResp {
    pub fn from_user(user: &User) -> Self {
        UserResp {
            uuid: user.uuid.simple().to_string(),
            username: user.username.clone(),
            role: user.role.clone(),
            email: user.email.clone(),
        }
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
    github(GithubLoginRequest),
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
        AuthUserRequest::github(g) => {
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
            // return it, if not,
            match User::from_partial_user(&conn, &pu) {
                Ok(u) => {
                    cookies.add_private(Cookie::new(
                        "user_uuid".to_owned(),
                        u.uuid.simple().to_string(),
                    ));
                    Json(Ok(AuthUserResp::UserResp(UserResp::from_user(&u))))
                }
                Err(e) => {
                    // TODO: better error handling for client vs server errs
                    Json(Ok(AuthUserResp::PartialUser(pu)))
                }
            }
        }
    }
}

#[get("/user", format = "application/json")]
pub fn get_user(user: User) -> Json<UserResp> {
    Json(UserResp::from_user(&user))
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

    // Compile-check that we can assume github's the only provider
    match req.partial_user.provider {
        oauth::Provider::Github => (),
    };

    // TODO: error handling, e.g. detect client vs server errors (such as uniqueness constraints
    // being client, and db conn errs being server)
    let create_res = (&*conn).transaction::<_, diesel::result::Error, _>(|| {
        use diesel;
        use diesel::prelude::*;
        use schema::users::dsl::*;
        use schema::github_accounts::dsl::*;
        use models::{NewGithubAccount, NewUser};
        let newuser: User = diesel::insert(&NewUser {
            username: &req.username,
            email: &req.email,
        }).into(users)
            .get_result(&*conn)?;

        diesel::insert(&NewGithubAccount {
            id: req.partial_user.provider_id,
            user_id: newuser._id,
            access_token: &req.partial_user.access_token,
        }).into(github_accounts)
            .execute(&*conn)?;

        Ok((newuser))
    });
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
