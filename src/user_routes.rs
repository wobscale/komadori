use std;
use db;
use oauth;
use diesel;
use diesel::Connection;
use rocket;
use uuid::Uuid;
use rocket::response::Failure;
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

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> rocket::request::Outcome<User, ()> {
        let mut cookies = request.cookies();
        let cookie_uuid = match cookies.get_private("user_uuid") {
            Some(uuid) => {
                match Uuid::parse_str(&uuid.value()) {
                    Ok(u) => u,
                    Err(e) => {
                        error!("could not decode user's uuid: {}", e);
                        return Outcome::Failure((Status::InternalServerError, ()));
                    }
                }
            }
            None => {
                return Outcome::Forward(());
            }
        };
        let db = request.guard::<rocket::State<db::Pool>>()?;
        let db = match db.get() {
            Ok(db) => db,
            Err(e) => {
                error!("error getting db: {}", e);
                return Outcome::Failure((Status::InternalServerError, ()));
            },
        };
        {
            use diesel::prelude::*;
            use schema::users::dsl::*;
            match users.filter(uuid.eq(cookie_uuid))
                .limit(1)
                .load::<User>(&*db) {
                    Ok(u) => {
                        match u.first() {
                            Some(u) =>  {
                                return Outcome::Success(u.clone());
                            },
                            None => {
                                error!("error getting user {}", cookie_uuid);
                                return Outcome::Failure((Status::InternalServerError, ()));
                            }
                        }
                    },
                    Err(e) => {
                        error!("error getting user {}", cookie_uuid);
                        return Outcome::Failure((Status::InternalServerError, ()));
                    }
                }
        }
        return Outcome::Forward(());
    }
}

#[get("/", rank = 0)]
fn index_user(user: User) -> String {
    format!("Hello {}", user.username)
}

pub struct Partialuser {
    provider: String,
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
            },
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
                        error!("could not create client");
                        return Outcome::Failure((Status::InternalServerError, ()));
                    }
                };

                #[derive(Deserialize)]
                struct GithubUser {
                    email: Option<String>,
                    name: Option<String>,
                    login: String,
                    id: i32,
                    avatar_url: Option<String>,
                }

                let user = match client.get().user().execute::<GithubUser>() {
                    Err(e) => {
                        error!("could not get github user");
                        return Outcome::Failure((Status::InternalServerError, ()));
                    },
                    Ok((headers, status, None)) => {
                        return Outcome::Failure((Status::InternalServerError, ()));
                    },
                    Ok((headers, status, Some(u))) => u,
                };
                (user.id, user.login)
            }
        };
        Outcome::Success(Partialuser{
            provider: token.provider.to_string(),
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
    ctx.add("oauth_provider", &user.provider);
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

    // TODO: error handling
    let create_res = (&*conn).transaction::<_, diesel::result::Error, _>(|| {
        use diesel;
        use diesel::prelude::*;
        use schema::users::dsl::*;
        use schema::github_accounts::dsl::*;
        use models::{NewUser, NewGithubAccount};
        let newuser: User = diesel::insert(&NewUser{
            username: &req.username,
            email: &req.email,
        }).into(users).get_result(&*conn)?;

        diesel::insert(&NewGithubAccount{
            id: user.provider_id,
            user_id: newuser._id,
            access_token: &user.access_token,
        }).into(github_accounts).execute(&*conn)?;

        Ok((newuser))
    });
    match create_res {
        Ok(newuser) => {
            cookies.add_private(Cookie::new("user_uuid".to_owned(), newuser.uuid.simple().to_string()));
            Flash::success(Redirect::to("/"), "Account created")
        },
        Err(e) => {
            error!("error creating user account: {}", e);
            Flash::error(Redirect::to("/"), "Could not create account for some reason")
        }
    }
}
