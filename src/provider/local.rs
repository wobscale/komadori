// The local provider implements an insecure provider meant for local development and
// integration testing.
//
// It should *NEVER* be used in production.

use super::{OauthData, OauthProvider};
use errors::Error;
use oauth;
use oauth2::Config;
use rocket;
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
        routes![]
    }
}

#[derive(Deserialize)]
pub struct LocalUser {
    pub login: String,
}

