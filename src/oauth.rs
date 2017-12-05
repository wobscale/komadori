use rocket;
use serde_json;
use rocket::request::{FromRequest, Request};
use rocket::http::Status;
use rocket::Outcome;
use oauth2;

// Copy of oauth2::Token so we can derive for it
#[derive(Serialize, Deserialize)]
#[serde(remote = "oauth2::Token")]
pub struct Token {
    pub token_type: String,
    pub access_token: String,
    pub scopes: Vec<String>,
    pub expires_in: Option<u32>,
    pub refresh_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Provider {
    Github,
}

impl Provider {
    pub fn to_string(&self) -> String {
        match *self {
            Provider::Github => "Github".to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerializableToken {
    pub provider: Provider,
    #[serde(with = "Token")]
    pub token: oauth2::Token,
}

impl<'a, 'r> FromRequest<'a, 'r> for SerializableToken {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> rocket::request::Outcome<SerializableToken, ()> {
        let mut cookies = request.cookies();

        let token_json = match cookies.get_private("oauth_token") {
            Some(c) => c,
            None => {
                return Outcome::Forward(());
            }
        };
        let token_json = token_json.value();

        let token: SerializableToken = match serde_json::from_str(token_json) {
            Ok(t) => t,
            Err(e) => {
                error!("could not deserialize token: {}", e);
                return Outcome::Failure((Status::InternalServerError, ()));
            }
        };

        return Outcome::Success(token);
    }
}
