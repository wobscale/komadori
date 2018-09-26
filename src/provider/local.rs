// The local provider implements an insecure provider meant for local development and
// integration testing.
//
// It should *NEVER* be used in production.

use super::{OauthData, OauthProvider};
use errors::Error;
use oauth;
use rocket;
use types;

#[derive(Debug, Clone)]
pub struct Local { }

impl Local {
    pub fn new() -> Self {
        Local {}
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
    fn routes(&self) -> Vec<rocket::Route> {
        routes![]
    }
}

#[derive(Deserialize)]
pub struct LocalUser {
    pub login: String,
}

