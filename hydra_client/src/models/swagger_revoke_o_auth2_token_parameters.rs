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
pub struct SwaggerRevokeOAuth2TokenParameters {
  /// in: formData
  #[serde(rename = "token")]
  token: String
}

impl SwaggerRevokeOAuth2TokenParameters {
  pub fn new(token: String) -> SwaggerRevokeOAuth2TokenParameters {
    SwaggerRevokeOAuth2TokenParameters {
      token: token
    }
  }

  pub fn set_token(&mut self, token: String) {
    self.token = token;
  }

  pub fn with_token(mut self, token: String) -> SwaggerRevokeOAuth2TokenParameters {
    self.token = token;
    self
  }

  pub fn token(&self) -> &String {
    &self.token
  }


}



