use oauth;
use oauth2::Config;
use rocket::http::{Cookie, Cookies};
use rocket::State;
use rocket::response::Redirect;
use rocket::response::Flash;
use serde_json;
use rand::{thread_rng, Rng};
use rocket;

pub fn routes() -> Vec<rocket::Route> {
    routes![get_authorize_url, authorize_url, handle_login]
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

    pub fn config(&self) -> Config {
        let redir_url = format!("{}/{}", self.base_url.trim_right_matches('/'), "github/oauth");
        Config::new(
            self.client_id.clone(),
            self.client_secret.clone(),
            AUTH_URL,
            TOKEN_URL,
        ).add_scope("user:email")
            .set_redirect_url(redir_url)
    }
}

#[get("/oauth")]
pub fn authorize_url(mut cookies: Cookies, oauth: State<OauthConfig>) -> Redirect {
    let state: String = thread_rng().gen_ascii_chars().take(16).collect();
    cookies.add_private(Cookie::new("github_state".to_owned(), state.clone()));
    let oauth_url = oauth.config().set_state(state).authorize_url().to_string();

    Redirect::to(&oauth_url)
}

#[get("/authorize_url")]
pub fn get_authorize_url(mut cookies: Cookies, oauth: State<OauthConfig>) -> String {
    let state: String = thread_rng().gen_ascii_chars().take(16).collect();
    cookies.add_private(Cookie::new("github_state".to_owned(), state.clone()));
    let oauth_url = oauth.config().set_state(state).authorize_url().to_string();

    oauth_url
}

#[derive(FromForm)]
struct GithubLoginRequest {
    code: String,
    state: String,
}

#[get("/login?<data>")]
fn handle_login(
    data: GithubLoginRequest,
    mut cookies: Cookies,
    oauth: State<OauthConfig>,
) -> Flash<Redirect> {
    let state = match cookies.get_private("github_state") {
        Some(c) => c,
        None => {
            return Flash::error(Redirect::to("/"), "missing state cookie");
        }
    };
    cookies.remove_private(Cookie::named("github_state"));
    let state_val = state.value();
    if data.state != state_val {
        return Flash::error(Redirect::to("/"), "state mismatch");
    }

    let token = match oauth.config().exchange_code(data.code) {
        Ok(t) => t,
        Err(e) => {
            error!("error exchanging code for a cookie: {}", e);
            return Flash::error(
                Redirect::to("/"),
                "unable to exchange access code for a cookie",
            );
        }
    };

    let oauth_token_json = match serde_json::to_string(&oauth::SerializableToken {
        provider: oauth::Provider::Github,
        token: token,
    }) {
        Err(e) => {
            error!("error serializing: {}", e);
            return Flash::error(Redirect::to("/"), "Internal Server Error!");
        }
        Ok(t) => t,
    };

    cookies.add_private(Cookie::new("oauth_token".to_string(), oauth_token_json));
    // We have safely saved the token in a place the login route can retrieve it
    // Our work here is done
    Flash::success(Redirect::to("/"), "Logged In")
}
