extern crate hydra_client;
extern crate hyper;
extern crate url;
extern crate oauth2;

use hydra_client::apis::client::APIClient;
use hydra_client::apis::configuration::Configuration;
use hyper::client::Client as HTTPClient;
use std::sync::Mutex;

pub struct HydraClientWrapper<C: hyper::client::Connect + Clone> {
    client: HTTPClient<C>,
    base_path: String,
    oauth_config: oauth2::Config,
    token: Mutex<oauth2::Token>,
}

impl<C: hyper::client::Connect + Clone> HydraClientWrapper<C> {
    pub fn new(client: HTTPClient<C>, base_path: &str, client_id: String, client_secret: String) -> Self {
        let bp = base_path.trim_right_matches("/");
        let mut oauth = oauth2::Config::new(client_id, client_secret, &format!("{}/oauth2/auth", bp), &format!("{}/oauth2/token", bp));
        oauth = oauth.add_scope("hydra.*");
        oauth = oauth.set_auth_type(oauth2::AuthType::BasicAuth);
        let token = oauth.exchange_client_credentials().unwrap();
        HydraClientWrapper{
            client: client,
            base_path: bp.to_owned(),
            oauth_config: oauth,
            token: Mutex::new(token),
        }
    }

    pub fn client(&self) -> APIClient<C> {
        let base_path = self.base_path.clone();
        let mut config = Configuration::new(self.client.clone());
        config.base_path = base_path;
        config.oauth_access_token = Some(self.ensure_token());
        APIClient::new(config)
    }

    fn ensure_token(&self) -> String {
        // TODO: refresh only if it needs to refresh
        let token2 = self.oauth_config.exchange_client_credentials().unwrap();
        let mut token = self.token.lock().unwrap();
        *token = token2;

        (*token.access_token).to_owned()
    }

}
