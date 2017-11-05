use oauth;
use oauth2::Config;
use rocket::http::{Cookie, Cookies};
use rocket_contrib::{Json, Value};
use rocket::State;
use serde_json;
use rand::{thread_rng, Rng};
use rocket;
use errors::Error;

pub fn routes() -> Vec<rocket::Route> {
    routes![authorize_url, handle_login]
}

pub struct OauthConfig {
    client_id: String,
    client_secret: String,
    base_url: String,
}

const AUTH_URL: &str = "https://github.com/login/oauth/authorize";
const TOKEN_URL: &str = "https://github.com/login/oauth/access_token";

impl OauthConfig {
    pub fn new(id: String, secret: String, base_url: String) -> Self {
        OauthConfig {
            client_id: id,
            client_secret: secret,
            base_url: base_url,
        }
    }

    fn config(&self) -> Config {
        let redir_url = format!("{}/{}", self.base_url.trim_right_matches('/'), "login");
        Config::new(
            self.client_id.clone(),
            self.client_secret.clone(),
            AUTH_URL,
            TOKEN_URL,
        ).add_scope("user:email")
            .set_redirect_url(redir_url)
    }
}

#[get("/authorize_url")]
pub fn authorize_url(mut cookies: Cookies, oauth: State<OauthConfig>) -> String {
    let state: String = thread_rng().gen_ascii_chars().take(16).collect();
    cookies.add_private(Cookie::new("github_state".to_owned(), state.clone()));

    oauth.config().set_state(state).authorize_url().to_string()
}

#[derive(FromForm)]
struct GithubLoginRequest {
    code: String,
    state: String,
}

struct GithubLoginResponse {}

impl<'a> rocket::response::Responder<'a> for GithubLoginResponse {
    fn respond_to(self, _: &rocket::Request) -> Result<rocket::Response<'a>, rocket::http::Status> {
        rocket::Response::build()
            .header(rocket::http::ContentType::JSON)
            .ok()
    }
}


#[get("/login?<data>")]
fn handle_login(
    data: GithubLoginRequest,
    mut cookies: Cookies,
    oauth: State<OauthConfig>,
) -> Result<GithubLoginResponse, Error> {
    let state = cookies.get_private("github_state").ok_or(
        Error::client_error(
            "missing state parameter"
                .to_owned(),
        ),
    )?;
    let state_val = state.value();
    if data.state != state_val {
        return Err(Error::client_error("state mismatch".to_owned()));
    }

    let token = oauth.config().exchange_code(data.code).map_err(|e| {
        error!("error exchanging code for a cookie: {}", e);
        Error::server_error("unable to exchange access code for a cookie".to_owned())
    })?;

    let oauth_token_json = serde_json::to_string(&oauth::SerializableToken {
        kind: "github".to_string(),
        token: token,
    }).map_err(|e| {
        error!("error serializing: {}", e);
        Error::server_error("error serializing token".to_owned())
    })?;

    cookies.add_private(Cookie::new("oauth_token".to_string(), oauth_token_json));
    // We have safely saved the token in a place the login route can retrieve it
    // Our work here is done
    Ok(GithubLoginResponse {})
}
