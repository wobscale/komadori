use reqwest;
use serde_json;
use serde;
use oauth2;
use uuid::Uuid;
use std::sync::Mutex;

pub struct Client {
    base_url: String,
    client_id: String,
    client_secret: String,
    token: Mutex<Option<oauth2::Token>>,
    client: reqwest::Client,
}

pub struct ClientToken {
    pub client_id: String,
    pub client_secret: String,
}

impl Client {
    pub fn new(base_url: &str, client_id: &str, client_secret: &str) -> Self {
        Client {
            base_url: base_url.trim_right_matches("/").to_string(),
            client_id: client_id.clone().to_string(),
            client_secret: client_secret.clone().to_string(),
            token: Mutex::new(None),
            client: reqwest::Client::new(),
        }
    }

    fn refresh_token(&self) -> Result<(), String> {
        let mut req = self.client.post(&format!("{}/oauth2/token", self.base_url));
        let res = req.basic_auth(self.client_id.clone(), Some(self.client_secret.clone()))
            .form(&vec![
                ("grant_type", "client_credentials"),
                ("scope", "hydra.*"),
            ])
            .send();
        let json_data = res.map_err(|e| format!("error refreshing: {}", e))?;

        let token: oauth2::Token = serde_json::from_reader(json_data)
            .map_err(|e| format!("Could not deserialize: {}", e))?;

        let mut self_token = self.token.lock().unwrap();
        *self_token = Some(token);
        Ok(())
    }

    fn make_request<S: serde::Serialize, D>(
        &self,
        method: reqwest::Method,
        path: &str,
        body: S,
    ) -> Result<D, String>
    where
        for<'de> D: serde::Deserialize<'de>,
    {
        let token = { self.token.lock().unwrap().clone() };
        if token.is_none() {
            self.refresh_token()?;
        }
        let token = { self.token.lock().unwrap().clone().unwrap().access_token };
        // TODO: refresh token if there's an error related to it
        let mut req = self.client
            .request(method, &format!("{}/{}", self.base_url, path));
        let mut res = req.json(&body)
            .header(reqwest::header::Authorization(reqwest::header::Bearer {
                token: token,
            }))
            .send()
            .map_err(|e| format!("error sending request to {}, {}", path, e))?;

        res.json()
            .map_err(|e| format!("could not decode response as json: {}", e))
    }

    fn make_nonjson_request<S: serde::Serialize>(
        &self,
        method: reqwest::Method,
        path: &str,
        body: S,
    ) -> Result<(), String> {
        let token = { self.token.lock().unwrap().clone() };
        if token.is_none() {
            self.refresh_token()?;
        }
        let token = { self.token.lock().unwrap().clone().unwrap().access_token };
        // TODO: refresh token if there's an error related to it
        let mut req = self.client
            .request(method, &format!("{}/{}", self.base_url, path));
        let res = req.json(&body)
            .header(reqwest::header::Authorization(reqwest::header::Bearer {
                token: token,
            }))
            .send()
            .map_err(|e| format!("error sending request to {}, {}", path, e))?;

        if !res.status().is_success() {
            return Err(format!("error status: {}", res.status()));
        }

        Ok(())
    }

    pub fn create_user(&self, name: &str) -> Result<ClientToken, String> {
        let resp: HydraCreateUserResp = self.make_request(
            reqwest::Method::Post,
            "clients",
            HydraCreateUserRequest {
                client_name: name.to_string(),
            },
        ).unwrap();

        Ok(ClientToken {
            client_id: resp.id,
            client_secret: resp.client_secret,
        })
    }

    pub fn consent_get(&self, id: &str) -> Result<ConsentInfoResponse, String> {
        let url = format!("oauth2/consent/requests/{}", id);
        let resp: ConsentInfoResponse = self.make_request(reqwest::Method::Get, &url, "").unwrap();

        Ok(resp)
    }

    pub fn consent_accept(&self, id: &str, scopes: &Vec<String>, user: Uuid) -> Result<(), String> {
        let url = format!("oauth2/consent/requests/{}/accept", id);
        self.make_nonjson_request(
            reqwest::Method::Patch,
            &url,
            HydraConsentnAllowRequest {
                subject: user.simple().to_string(),
                grant_scopes: scopes.clone(),
            },
        )
    }

    pub fn consent_reject(&self, id: &str, reason: &str) -> Result<(), String> {
        let url = format!("oauth2/consent/requests/{}/reject", id);
        self.make_request(
            reqwest::Method::Patch,
            &url,
            HydraConsentnRejectRequest {
                reason: reason.to_string(),
            },
        )
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConsentInfoResponse {
    pub requested_scopes: Vec<String>,
    pub client_id: String,
    pub id: String,
    pub redirect_url: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct HydraConsentnAllowRequest {
    grant_scopes: Vec<String>,
    subject: String,
}

#[derive(Serialize)]
struct HydraConsentnRejectRequest {
    reason: String,
}

#[derive(Serialize)]
struct HydraCreateUserRequest {
    client_name: String,
}

#[derive(Deserialize)]
struct HydraCreateUserResp {
    // There are other fields which I'm ignoring for now
    id: String,
    client_secret: String,
}
