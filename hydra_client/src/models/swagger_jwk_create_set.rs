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
pub struct SwaggerJwkCreateSet {
  #[serde(rename = "Body")]
  body: Option<::models::JsonWebKeySetGeneratorRequest>,
  /// The set in: path
  #[serde(rename = "set")]
  set: String
}

impl SwaggerJwkCreateSet {
  pub fn new(set: String) -> SwaggerJwkCreateSet {
    SwaggerJwkCreateSet {
      body: None,
      set: set
    }
  }

  pub fn set_body(&mut self, body: ::models::JsonWebKeySetGeneratorRequest) {
    self.body = Some(body);
  }

  pub fn with_body(mut self, body: ::models::JsonWebKeySetGeneratorRequest) -> SwaggerJwkCreateSet {
    self.body = Some(body);
    self
  }

  pub fn body(&self) -> Option<&::models::JsonWebKeySetGeneratorRequest> {
    self.body.as_ref()
  }

  pub fn reset_body(&mut self) {
    self.body = None;
  }

  pub fn set_set(&mut self, set: String) {
    self.set = set;
  }

  pub fn with_set(mut self, set: String) -> SwaggerJwkCreateSet {
    self.set = set;
    self
  }

  pub fn set(&self) -> &String {
    &self.set
  }


}



