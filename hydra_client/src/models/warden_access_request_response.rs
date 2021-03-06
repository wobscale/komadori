/* 
 * ORY Hydra - Cloud Native OAuth 2.0 and OpenID Connect Server
 *
 * Welcome to the ORY Hydra HTTP API documentation. You will find documentation for all HTTP APIs here. Keep in mind that this document reflects the latest branch, always. Support for versioned documentation is coming in the future.
 *
 * OpenAPI spec version: Latest
 * Contact: hi@ory.am
 * Generated by: https://openapi-generator.tech
 */

/// WardenAccessRequestResponse : The warden access request response

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct WardenAccessRequestResponse {
  /// Allowed is true if the request is allowed and false otherwise.
  #[serde(rename = "allowed")]
  allowed: Option<bool>
}

impl WardenAccessRequestResponse {
  /// The warden access request response
  pub fn new() -> WardenAccessRequestResponse {
    WardenAccessRequestResponse {
      allowed: None
    }
  }

  pub fn set_allowed(&mut self, allowed: bool) {
    self.allowed = Some(allowed);
  }

  pub fn with_allowed(mut self, allowed: bool) -> WardenAccessRequestResponse {
    self.allowed = Some(allowed);
    self
  }

  pub fn allowed(&self) -> Option<&bool> {
    self.allowed.as_ref()
  }

  pub fn reset_allowed(&mut self) {
    self.allowed = None;
  }

}



