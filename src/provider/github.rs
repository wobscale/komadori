use github_rs::client::{Github as GHClient, Executor};
use oauth2::Config;
use super::{OauthProvider, OauthData, ProviderSet};
use rand::{thread_rng, Rng};
use oauth;
use rocket::http::{Cookie, Cookies};
use rocket::State;
use errors::Error;
use types;
use rocket;

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

}

impl OauthProvider for Github {
    fn routes(&self) -> Vec<rocket::Route> {
        routes![authorize_url]
    }
}

#[get("/github/authorize_url")]
pub fn authorize_url(mut cookies: Cookies, provider: State<ProviderSet>) -> Result<String, String> {
    let provider = match &provider.github {
        Some(p) => p,
        None => {
            return Err("Provider not configured".to_string());
        }
    };
    let state: String = thread_rng().gen_ascii_chars().take(16).collect();
    cookies.add_private(Cookie::new("github_state".to_owned(), state.clone()));
    let oauth_url = provider.config().set_state(state).authorize_url().to_string();

    Ok(oauth_url)
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
