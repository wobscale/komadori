// The local provider implements an insecure provider meant for local development and
// integration testing.
//
// It should *NEVER* be used in production.

use super::{OauthData, OauthProvider};
use db;
use errors::Error;
use oauth;
use oauth2::Config;
use rocket;
use rocket::http::{Cookie, Cookies};
use rocket::State;
use rocket_contrib::json::Json;
use std::collections::HashMap;
use types;

#[derive(Debug, Clone)]
pub struct Local {
    base_url: String,
}

impl Local {
    pub fn new(base_url: String) -> Self {
        Local {
            base_url: base_url,
        }
    }

    pub fn partial_user(&self, data: &OauthData) -> Result<types::PartialUser, Error> {
        let parts: Vec<_> = data.state.split(" ").collect();
        Ok(types::PartialUser {
            provider: oauth::Provider::Local,
            provider_id: parts[0].parse().unwrap(),
            provider_name: parts[1].to_string(),
            access_token: data.code.to_string(),
        })
    }
}

impl OauthProvider for Local {
    fn config(&self) -> Config {
        let redir_url = format!(
            "{}/{}",
            self.base_url.trim_right_matches('/'),
            "dev/local/oauth"
        );
        Config::new(
            "insecure_id".clone(),
            "insecure_secret".clone(),
            self.base_url.clone() + "/dev/local/auth",
            self.base_url.clone() + "/dev/local/token",
        ).add_scope("user:email")
        .set_redirect_url(redir_url)
    }

    fn routes(&self) -> Vec<rocket::Route> {
        routes![local_authorize_url, local_auth]
    }
}

#[get("/dev/local/authorize_url")]
pub fn local_authorize_url(mut cookies: Cookies, provider: State<Local>) -> Result<String, String> {
    let provider = provider.inner();
    let oauth_url = provider
        .config()
        .set_state("insecure-state".to_string())
        .authorize_url()
        .to_string();

    Ok(oauth_url)
}

#[get("/dev/local/auth")]
pub fn local_auth(mut cookies: Cookies, provider: State<Local>) -> Result<String, String> {
    let provider = provider.inner();
    let state: String = "insecure-state".to_string();
    cookies.add_private(Cookie::new("insecure_state".to_owned(), state.clone()));
    let oauth_url = provider
        .config()
        .set_state(state)
        .authorize_url()
        .to_string();

    Ok(oauth_url)
}

#[derive(Deserialize)]
pub struct LocalUser {
    pub login: String,
}

