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
pub struct SwaggerUpdatePolicyParameters {
  #[serde(rename = "Body")]
  body: Option<::models::Policy>,
  /// The id of the policy. in: path
  #[serde(rename = "id")]
  id: Option<String>
}

impl SwaggerUpdatePolicyParameters {
  pub fn new() -> SwaggerUpdatePolicyParameters {
    SwaggerUpdatePolicyParameters {
      body: None,
      id: None
    }
  }

  pub fn set_body(&mut self, body: ::models::Policy) {
    self.body = Some(body);
  }

  pub fn with_body(mut self, body: ::models::Policy) -> SwaggerUpdatePolicyParameters {
    self.body = Some(body);
    self
  }

  pub fn body(&self) -> Option<&::models::Policy> {
    self.body.as_ref()
  }

  pub fn reset_body(&mut self) {
    self.body = None;
  }

  pub fn set_id(&mut self, id: String) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: String) -> SwaggerUpdatePolicyParameters {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&String> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

}



