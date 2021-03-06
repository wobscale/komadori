/* 
 * ORY Hydra - Cloud Native OAuth 2.0 and OpenID Connect Server
 *
 * Welcome to the ORY Hydra HTTP API documentation. You will find documentation for all HTTP APIs here. Keep in mind that this document reflects the latest branch, always. Support for versioned documentation is coming in the future.
 *
 * OpenAPI spec version: Latest
 * Contact: hi@ory.am
 * Generated by: https://openapi-generator.tech
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct OAuth2TokenIntrospection {
  /// Active is a boolean indicator of whether or not the presented token is currently active.  The specifics of a token's \"active\" state will vary depending on the implementation of the authorization server and the information it keeps about its tokens, but a \"true\" value return for the \"active\" property will generally indicate that a given token has been issued by this authorization server, has not been revoked by the resource owner, and is within its given time window of validity (e.g., after its issuance time and before its expiration time).
  #[serde(rename = "active")]
  active: Option<bool>,
  /// ClientID is a service-specific string identifier or list of string identifiers representing the intended audience for this token.
  #[serde(rename = "aud")]
  aud: Option<String>,
  /// ClientID is aclient identifier for the OAuth 2.0 client that requested this token.
  #[serde(rename = "client_id")]
  client_id: Option<String>,
  /// Expires at is an integer timestamp, measured in the number of seconds since January 1 1970 UTC, indicating when this token will expire.
  #[serde(rename = "exp")]
  exp: Option<i64>,
  /// Extra is arbitrary data set by the session.
  #[serde(rename = "ext")]
  ext: Option<::std::collections::HashMap<String, Value>>,
  /// Issued at is an integer timestamp, measured in the number of seconds since January 1 1970 UTC, indicating when this token was originally issued.
  #[serde(rename = "iat")]
  iat: Option<i64>,
  /// Issuer is a string representing the issuer of this token
  #[serde(rename = "iss")]
  iss: Option<String>,
  /// NotBefore is an integer timestamp, measured in the number of seconds since January 1 1970 UTC, indicating when this token is not to be used before.
  #[serde(rename = "nbf")]
  nbf: Option<i64>,
  /// Scope is a JSON string containing a space-separated list of scopes associated with this token.
  #[serde(rename = "scope")]
  scope: Option<String>,
  /// Subject of the token, as defined in JWT [RFC7519]. Usually a machine-readable identifier of the resource owner who authorized this token.
  #[serde(rename = "sub")]
  sub: Option<String>,
  /// Username is a human-readable identifier for the resource owner who authorized this token.
  #[serde(rename = "username")]
  username: Option<String>
}

impl OAuth2TokenIntrospection {
  pub fn new() -> OAuth2TokenIntrospection {
    OAuth2TokenIntrospection {
      active: None,
      aud: None,
      client_id: None,
      exp: None,
      ext: None,
      iat: None,
      iss: None,
      nbf: None,
      scope: None,
      sub: None,
      username: None
    }
  }

  pub fn set_active(&mut self, active: bool) {
    self.active = Some(active);
  }

  pub fn with_active(mut self, active: bool) -> OAuth2TokenIntrospection {
    self.active = Some(active);
    self
  }

  pub fn active(&self) -> Option<&bool> {
    self.active.as_ref()
  }

  pub fn reset_active(&mut self) {
    self.active = None;
  }

  pub fn set_aud(&mut self, aud: String) {
    self.aud = Some(aud);
  }

  pub fn with_aud(mut self, aud: String) -> OAuth2TokenIntrospection {
    self.aud = Some(aud);
    self
  }

  pub fn aud(&self) -> Option<&String> {
    self.aud.as_ref()
  }

  pub fn reset_aud(&mut self) {
    self.aud = None;
  }

  pub fn set_client_id(&mut self, client_id: String) {
    self.client_id = Some(client_id);
  }

  pub fn with_client_id(mut self, client_id: String) -> OAuth2TokenIntrospection {
    self.client_id = Some(client_id);
    self
  }

  pub fn client_id(&self) -> Option<&String> {
    self.client_id.as_ref()
  }

  pub fn reset_client_id(&mut self) {
    self.client_id = None;
  }

  pub fn set_exp(&mut self, exp: i64) {
    self.exp = Some(exp);
  }

  pub fn with_exp(mut self, exp: i64) -> OAuth2TokenIntrospection {
    self.exp = Some(exp);
    self
  }

  pub fn exp(&self) -> Option<&i64> {
    self.exp.as_ref()
  }

  pub fn reset_exp(&mut self) {
    self.exp = None;
  }

  pub fn set_ext(&mut self, ext: ::std::collections::HashMap<String, Value>) {
    self.ext = Some(ext);
  }

  pub fn with_ext(mut self, ext: ::std::collections::HashMap<String, Value>) -> OAuth2TokenIntrospection {
    self.ext = Some(ext);
    self
  }

  pub fn ext(&self) -> Option<&::std::collections::HashMap<String, Value>> {
    self.ext.as_ref()
  }

  pub fn reset_ext(&mut self) {
    self.ext = None;
  }

  pub fn set_iat(&mut self, iat: i64) {
    self.iat = Some(iat);
  }

  pub fn with_iat(mut self, iat: i64) -> OAuth2TokenIntrospection {
    self.iat = Some(iat);
    self
  }

  pub fn iat(&self) -> Option<&i64> {
    self.iat.as_ref()
  }

  pub fn reset_iat(&mut self) {
    self.iat = None;
  }

  pub fn set_iss(&mut self, iss: String) {
    self.iss = Some(iss);
  }

  pub fn with_iss(mut self, iss: String) -> OAuth2TokenIntrospection {
    self.iss = Some(iss);
    self
  }

  pub fn iss(&self) -> Option<&String> {
    self.iss.as_ref()
  }

  pub fn reset_iss(&mut self) {
    self.iss = None;
  }

  pub fn set_nbf(&mut self, nbf: i64) {
    self.nbf = Some(nbf);
  }

  pub fn with_nbf(mut self, nbf: i64) -> OAuth2TokenIntrospection {
    self.nbf = Some(nbf);
    self
  }

  pub fn nbf(&self) -> Option<&i64> {
    self.nbf.as_ref()
  }

  pub fn reset_nbf(&mut self) {
    self.nbf = None;
  }

  pub fn set_scope(&mut self, scope: String) {
    self.scope = Some(scope);
  }

  pub fn with_scope(mut self, scope: String) -> OAuth2TokenIntrospection {
    self.scope = Some(scope);
    self
  }

  pub fn scope(&self) -> Option<&String> {
    self.scope.as_ref()
  }

  pub fn reset_scope(&mut self) {
    self.scope = None;
  }

  pub fn set_sub(&mut self, sub: String) {
    self.sub = Some(sub);
  }

  pub fn with_sub(mut self, sub: String) -> OAuth2TokenIntrospection {
    self.sub = Some(sub);
    self
  }

  pub fn sub(&self) -> Option<&String> {
    self.sub.as_ref()
  }

  pub fn reset_sub(&mut self) {
    self.sub = None;
  }

  pub fn set_username(&mut self, username: String) {
    self.username = Some(username);
  }

  pub fn with_username(mut self, username: String) -> OAuth2TokenIntrospection {
    self.username = Some(username);
    self
  }

  pub fn username(&self) -> Option<&String> {
    self.username.as_ref()
  }

  pub fn reset_username(&mut self) {
    self.username = None;
  }

}



