use std;
use db;
use oauth;
use diesel;
use diesel::Connection;
use rocket;
use std::time::Instant;
use uuid::Uuid;
use rocket::http::Status;
use rocket::response::{Flash, Redirect};
use rocket_contrib::Template;
use rocket::Outcome;
use rocket::request::{FromRequest, Request, Form};
use rocket::http::{Cookie, Cookies};
use github_rs::client::Executor;
use github_rs::client::Github;
use tera;

pub fn routes() -> Vec<rocket::Route> {
    routes![create_user, index_user, index_user_creating]
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
            Ok(u) => {
                match u.first() {
                    Some(u) => Ok(u.clone()),
                    None => {
                        error!("error getting user {}", uuid_);
                        Err(GetUserError::NoSuchUser)
                    }
                }
            }
            Err(e) => {
                error!("error getting user {}", uuid_);
                Err(GetUserError::DbError(e))
            }
        }
    }
    fn from_partial_user(
        conn: &diesel::PgConnection,
        pu: Partialuser,
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
            Ok(u) => {
                match u.first() {
                    Some(u) => Ok(u.clone()),
                    None => Err(GetUserError::NoSuchUser),
                }
            }
            Err(e) => Err(GetUserError::DbError(e)),
        }
    }
}


impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> rocket::request::Outcome<Self, ()> {
        let uuid_ = {
            // ensure cookies is dropped before the Partialuser::from_request so we don't error out
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
                match Partialuser::from_request(request) {
                    Outcome::Success(pu) => {
                        match User::from_partial_user(&*db, pu) {
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
                    Outcome::Forward(()) => {
                        return Outcome::Forward(())
                    }
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

#[get("/", rank = 0)]
fn index_user(user: User) -> String {
    format!("Hello {}", user.username)
}

pub struct Partialuser {
    provider: oauth::Provider,
    provider_id: i32,
    provider_name: String,
    access_token: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for Partialuser {
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
        Outcome::Success(Partialuser {
            provider: token.provider,
            provider_id: uid,
            provider_name: name,
            access_token: token.token.access_token,
        })
    }
}

#[get("/", rank = 1)]
fn index_user_creating(user: Partialuser) -> Template {
    let mut ctx = tera::Context::new();
    ctx.add("csrf", &"TODO");
    ctx.add("oauth_provider", &user.provider.to_string());
    ctx.add("oauth_account_name", &user.provider_name);
    Template::render("partial_user/index", ctx)
}

#[derive(Debug, Serialize, Deserialize, FromForm)]
pub struct CreateUserRequest {
    username: String,
    email: String,
}



#[post("/user/create", data = "<form>")]
pub fn create_user(
    conn: db::Conn,
    user: Partialuser,
    form: Form<CreateUserRequest>,
    mut cookies: Cookies,
) -> Flash<Redirect> {
    let req = form.get();
    if req.username.len() == 0 {
        return Flash::error(Redirect::to("/"), "Name cannot be blank");
    }
    if req.email.len() == 0 {
        return Flash::error(Redirect::to("/"), "Email cannot be blank");
    }

    // Compile-check that we can assume github's the only provider
    match user.provider {
        oauth::Provider::Github => (),
    };

    // TODO: error handling
    let create_res = (&*conn).transaction::<_, diesel::result::Error, _>(|| {
        use diesel;
        use diesel::prelude::*;
        use schema::users::dsl::*;
        use schema::github_accounts::dsl::*;
        use models::{NewUser, NewGithubAccount};
        let newuser: User = diesel::insert(&NewUser {
            username: &req.username,
            email: &req.email,
        }).into(users)
            .get_result(&*conn)?;

        diesel::insert(&NewGithubAccount {
            id: user.provider_id,
            user_id: newuser._id,
            access_token: &user.access_token,
        }).into(github_accounts)
            .execute(&*conn)?;

        Ok((newuser))
    });
    match create_res {
        Ok(newuser) => {
            cookies.add_private(Cookie::new(
                "user_uuid".to_owned(),
                newuser.uuid.simple().to_string(),
            ));
            Flash::success(Redirect::to("/"), "Account created")
        }
        Err(e) => {
            error!("error creating user account: {}", e);
            Flash::error(
                Redirect::to("/"),
                "Could not create account for some reason",
            )
        }
    }
}
