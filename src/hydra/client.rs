use hydra_oauthed_client;
use hyper;
use tokio_core;

pub struct ClientBuilder {
    base_url: String,
    client_id: String,
    client_secret: String,
}

impl ClientBuilder {
    pub fn new(base_url: &str, client_id: &str, client_secret: &str) -> Self {
        ClientBuilder {
            base_url: base_url.trim_right_matches("/").to_string(),
            client_id: client_id.clone().to_string(),
            client_secret: client_secret.clone().to_string(),
        }
    }

    pub fn build(&self, handle: &tokio_core::reactor::Handle) -> hydra_oauthed_client::HydraClientWrapper<hyper::client::HttpConnector> {
        let client = hyper::Client::new(handle);
        hydra_oauthed_client::HydraClientWrapper::new(client, &self.base_url.clone(), self.client_id.clone(), self.client_secret.clone())
    }
}
