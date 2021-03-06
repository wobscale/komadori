use db;
use provider::github::get_github_user;
use github_rs::client::Executor;
use github_rs::client::Github;
use oauth;
use errors::Error;
use hydra;
use multi_reactor_drifting;
use db::users::User as DBUser;
use multi_reactor_drifting::Handle;
use futures::Future;
use rocket;
use rocket::http::hyper::header::Bearer as BearerAuth;
use rocket::http::Status;
use rocket::request::{FromRequest, Request};
use rocket::Outcome;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct PartialUser {
    pub provider: oauth::Provider,
    pub provider_id: i32,
    pub provider_name: String,
    pub access_token: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct UserResp {
    pub uuid: String,
    pub username: String,
    pub role: Option<String>,
    pub email: String,

    pub groups: Vec<GroupResp>,

    pub auth_metadata: AuthMetadata,
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
                get_github_user(&c.access_token)
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


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct AuthMetadata {
    pub github: Option<Result<GithubAuthMetadata, String>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct GithubAuthMetadata {
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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


#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum AuthUserResp {
    UserResp(UserResp),
    PartialUser(PartialUser),
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
            oauth::Provider::Local => unimplemented!(),
        };
        Outcome::Success(PartialUser {
            provider: token.provider,
            provider_id: uid,
            provider_name: name,
            access_token: token.token.access_token,
        })
    }
}



pub type User = DBUser;

pub struct CookieUser(pub User);
impl<'a, 'r> FromRequest<'a, 'r> for CookieUser {
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
                                u.uuid
                            }
                            Err(e) => {
                                error!("could not create user from partial user: {:?}", e);
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
            Ok(u) => Outcome::Success(CookieUser(u)),
        }
    }
}

pub struct OauthUser {
    pub user: User,
    pub scopes: Vec<String>,
}

impl<'a, 'r> FromRequest<'a, 'r> for OauthUser {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> rocket::request::Outcome<Self, ()> {
        let raw_auth = match request.headers().get_one("authorization") {
            Some(s) => s,
            None => {
                debug!("No authorization header");
                return Outcome::Failure((Status::Unauthorized, ()));
            }
        };
        
        let auth: BearerAuth = match raw_auth.parse() {
            Ok(a) => a,
            Err(e) => {
                warn!("could not parse Authorization header: {:?}", e);
                return Outcome::Failure((Status::Unauthorized, ()));
            },
        };

        // split off the 'Bearer' prefix
        let token = auth.token.split_whitespace().last().unwrap_or("");
        if token == "" {
            return Outcome::Failure((Status::Unauthorized, ()));
        }

        let hydra = request.guard::<rocket::State<hydra::client::ClientBuilder>>()?;
        let handle = request.guard::<Handle>()?;
        let client = hydra.build(&handle.into()).client();

        let res = client.o_auth2_api().introspect_o_auth2_token(&token, "")
            .then(|res| {
                match res {
                    Ok(user) => {
                        if !user.active().unwrap_or(&false) {
                            return Err(Outcome::Failure((Status::Unauthorized, ())));
                        }
                        Ok((user.sub().unwrap().clone(), user.scope().unwrap().clone()))
                    }
                    Err(e) => {
                        error!("oauth2 introspect error: {:?}", e);
                        Err(Outcome::Failure((Status::InternalServerError, ())))
                    }
                }
            }).and_then(|(uuid, scopes)| {
                let db = match request.guard::<rocket::State<db::Pool>>() {
                    rocket::Outcome::Success(p) => p,
                    rocket::Outcome::Failure(f) => {
                        return Err(rocket::Outcome::Failure(f));
                    },
                    rocket::Outcome::Forward(f) => {
                        return Err(rocket::Outcome::Forward(f));
                    },
                };
                let db = match db.get() {
                    Ok(db) => db,
                    Err(e) => {
                        error!("error getting db: {}", e);
                        return Err(Outcome::Failure((Status::InternalServerError, ())));
                    }
                };

                let uuid: Uuid = match uuid.parse() {
                    Err(e) => {
                        warn!("uuid parse error: {:?}", e);
                        return Err(Outcome::Failure((Status::BadRequest, ())));
                    }
                    Ok(u) => u,
                };

                match User::from_uuid(&*db, uuid) {
                    Err(db::users::GetUserError::NoSuchUser) => {
                        Err(Outcome::Failure((Status::NotFound, ())))
                    },
                    Err(e) => {
                        error!("error using uuid to get user: {:?}", e);
                        Err(Outcome::Failure((Status::InternalServerError, ())))
                    }
                    Ok(u) => Ok(Outcome::Success(OauthUser{
                        user: u,
                        scopes: scopes.split_whitespace().map(String::from).collect(),
                    }))
                }
            });

        match multi_reactor_drifting::run(res) {
            Ok(o) => o,
            Err(e) => {
                e
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub partial_user: PartialUser,
    pub username: String,
    pub email: String,
}

