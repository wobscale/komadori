use rocket;
use oauth2::Config;
use types::PartialUser;
use errors::Error;

pub mod github;
pub mod local;

// Since there are a known number of implementers of OauthProvider, we can just create a poor-man's
// set by storing each as an optional field.
pub struct ProviderSet {
    pub github: Option<github::Github>,
    pub local: Option<local::Local>,
}

impl ProviderSet {
    pub fn partial_user(&self, req: &ProviderAuthRequest) -> Result<PartialUser, Error> {
        match req {
            ProviderAuthRequest::Github(data) => {
                let gh = match &self.github {
                    Some(g) => g,
                    None => {
                        return Err(Error::server_error("provider Github not configured".to_string()));
                    }
                };
                gh.partial_user(data)
            }
            ProviderAuthRequest::Local(data) => {
                let local = match &self.local {
                    Some(p) => p,
                    None => {
                        return Err(Error::server_error("provider Github not configured".to_string()));
                    }
                };
                local.partial_user(data)
            }
        }
    }

    pub fn routes(&self) -> Vec<rocket::Route> {
        let mut ret = Vec::new();
        match &self.github {
            Some(p) => {
                ret.extend(p.routes());
            }
            None => {},
        };
        match &self.local {
            Some(p) => {
                ret.extend(p.routes());
            }
            None => {},
        };

        ret
    }
}

#[derive(Deserialize)]
#[serde(tag = "provider")]
pub enum ProviderAuthRequest {
    #[serde(rename = "github")]
    Github(OauthData),
    #[serde(rename = "local")]
    Local(OauthData),
}

pub trait OauthProvider {
    fn config(&self) -> Config;
    fn routes(&self) -> Vec<rocket::Route>;
}

#[derive(Debug, Deserialize)]
pub struct OauthData {
    code: String,
    state: String,
}
