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

#[derive(Serialize, Deserialize)]
pub struct SerializableToken {
    pub kind: String,
    #[serde(with = "Token")]
    pub token: oauth2::Token,
}
