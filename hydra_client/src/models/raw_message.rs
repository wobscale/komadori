/* 
 * ORY Hydra - Cloud Native OAuth 2.0 and OpenID Connect Server
 *
 * Welcome to the ORY Hydra HTTP API documentation. You will find documentation for all HTTP APIs here. Keep in mind that this document reflects the latest branch, always. Support for versioned documentation is coming in the future.
 *
 * OpenAPI spec version: Latest
 * Contact: hi@ory.am
 * Generated by: https://openapi-generator.tech
 */

/// RawMessage : It implements Marshaler and Unmarshaler and can be used to delay JSON decoding or precompute a JSON encoding.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RawMessage {
}

impl RawMessage {
  /// It implements Marshaler and Unmarshaler and can be used to delay JSON decoding or precompute a JSON encoding.
  pub fn new() -> RawMessage {
    RawMessage {
    }
  }

}



