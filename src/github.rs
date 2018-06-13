use oauth2::Config;
use rocket::http::{Cookie, Cookies};
use rocket::State;
use rand::{thread_rng, Rng};
use rocket;

pub fn routes() -> Vec<rocket::Route> {
    routes![get_authorize_url]
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
}

#[get("/authorize_url")]
pub fn get_authorize_url(mut cookies: Cookies, oauth: State<OauthConfig>) -> String {
    let state: String = thread_rng().gen_ascii_chars().take(16).collect();
    cookies.add_private(Cookie::new("github_state".to_owned(), state.clone()));
    let oauth_url = oauth.config().set_state(state).authorize_url().to_string();

    oauth_url
}
