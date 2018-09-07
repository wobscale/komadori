use github_rs::client::{Github as GHClient, Executor};
use oauth2::Config;
use rocket_contrib::json::Json;
use super::{OauthProvider, OauthData};
use rand::{thread_rng, Rng};
use oauth;
use rocket::http::{Cookie, Cookies};
use rocket::State;
use errors::Error;
use types;
use rocket;
use db;

const AUTH_URL: &str = "https://github.com/login/oauth/authorize";
const TOKEN_URL: &str = "https://github.com/login/oauth/access_token";

#[derive(Debug, Clone)]
pub struct Github {
    client_id: String,
    client_secret: String,
    base_url: String,
}

impl Github {
    pub fn new(client_id: String, client_secret: String, base_url: String) -> Self {
        Github{
            client_id: client_id,
            client_secret: client_secret,
            base_url: base_url,
        }
    }

    pub fn partial_user(&self, data: &OauthData) -> Result<types::PartialUser, Error> {
        // We got github oauth tokens, exchange it for an access code
        let token = match self.config().exchange_code(data.code.clone()) {
            Ok(t) => t,
            Err(e) => {
                error!("github exchange code error: {}", e);
                return Err(Error::client_error(
                    "could not exchange code".to_string(),
                ));
            }
        };

        let user = match get_github_user(&token.access_token) {
            Ok(u) => u,
            Err(e) => return Err(e),
        };

        Ok(types::PartialUser {
            provider: oauth::Provider::Github,
            provider_id: user.id,
            provider_name: user.login,
            access_token: token.access_token,
        })
    }
}

impl OauthProvider for Github {
    fn config(&self) -> Config {
        let redir_url = format!(
            "{}/{}",
            self.base_url.trim_right_matches('/'),
            "github/oauth"
            );
        Config::new(
            self.client_id.clone(),
            self.client_secret.clone(),
            AUTH_URL,
            TOKEN_URL,
            ).add_scope("user:email")
            .set_redirect_url(redir_url)
    }

    fn routes(&self) -> Vec<rocket::Route> {
        routes![github_route, auth_user]
    }
}

#[get("/github/authorize_url")]
pub fn github_route(mut cookies: Cookies, provider: State<Github>) -> Result<String, String> {
    let provider = provider.inner();
    let state: String = thread_rng().gen_ascii_chars().take(16).collect();
    cookies.add_private(Cookie::new("github_state".to_owned(), state.clone()));
    let oauth_url = provider.config().set_state(state).authorize_url().to_string();

    Ok(oauth_url)
}


#[post("/github/auth", format = "application/json", data = "<oauth_data>")]
pub fn auth_user(
    conn: db::Conn,
    oauth_data: Json<OauthData>,
    github: State<Github>,
    mut cookies: Cookies,
) -> Json<Result<types::AuthUserResp, Error>> {
        // We got github oauth tokens, exchange it for an access code
        let token = match github.config().exchange_code(oauth_data.code.clone()) {
            Ok(t) => t,
            Err(e) => {
                error!("github exchange code error: {}", e);
                return Json(Err(Error::client_error(
                    "could not exchange code".to_string(),
                )));
            }
        };

        let user = match get_github_user(&token.access_token) {
            Ok(u) => u,
            Err(e) => return Json(Err(e)),
        };

        let pu = types::PartialUser {
            provider: oauth::Provider::Github,
            provider_id: user.id,
            provider_name: user.login,
            access_token: token.access_token,
        };

        // Now either this github account id could have an associated user, or not. If it does,
        // return it, if not, assume this is a partial user.
        match types::User::from_oauth_provider(&conn, &pu.provider, &pu.provider_id) {
            Ok(u) => {
                cookies.add_private(Cookie::new(
                    "user_uuid".to_owned(),
                    u.uuid.simple().to_string(),
                ));
                let ru = match types::UserResp::new(u, &conn) {
                    Err(e) => {
                        return Json(Err(e));
                    }
                    Ok(ru) => ru,
                };
                Json(Ok(types::AuthUserResp::UserResp(ru)))
            }
            Err(e) => {
                debug!("error getting user from partial user; assuming user doesn't exist: {:?}", e);
                // TODO: better error handling for client vs server errs
                Json(Ok(types::AuthUserResp::PartialUser(pu)))
            }
        }
}

#[derive(Deserialize)]
pub struct GithubUser {
    _email: Option<String>,
    _name: Option<String>,
    pub login: String,
    pub id: i32,
    _avatar_url: Option<String>,
}

pub fn get_github_user(access_token: &str) -> Result<GithubUser, Error> {
    let gh = GHClient::new(access_token)
        .map_err(|e| {
            Error::server_error(format!("could not create github client: {}", e))
        })?;

    let user = match gh.get().user().execute::<GithubUser>() {
        Err(e) => {
            error!("could not get github user: {}", e);
            return Err(Error::client_error(
                "could not get github user with token".to_string(),
            ));
        }
        Ok((_, _, None)) => {
            return Err(Error::server_error(
                "Github returned success, but with no user??".to_string(),
            ));
        }
        Ok((_, _, Some(u))) => u,
    };

    Ok(user)
}
