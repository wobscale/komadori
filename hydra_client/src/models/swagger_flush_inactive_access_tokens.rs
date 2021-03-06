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
pub struct SwaggerFlushInactiveAccessTokens {
  #[serde(rename = "Body")]
  body: Option<::models::FlushInactiveOAuth2TokensRequest>
}

impl SwaggerFlushInactiveAccessTokens {
  pub fn new() -> SwaggerFlushInactiveAccessTokens {
    SwaggerFlushInactiveAccessTokens {
      body: None
    }
  }

  pub fn set_body(&mut self, body: ::models::FlushInactiveOAuth2TokensRequest) {
    self.body = Some(body);
  }

  pub fn with_body(mut self, body: ::models::FlushInactiveOAuth2TokensRequest) -> SwaggerFlushInactiveAccessTokens {
    self.body = Some(body);
    self
  }

  pub fn body(&self) -> Option<&::models::FlushInactiveOAuth2TokensRequest> {
    self.body.as_ref()
  }

  pub fn reset_body(&mut self) {
    self.body = None;
  }

}



