/* 
 * ORY Hydra - Cloud Native OAuth 2.0 and OpenID Connect Server
 *
 * Welcome to the ORY Hydra HTTP API documentation. You will find documentation for all HTTP APIs here. Keep in mind that this document reflects the latest branch, always. Support for versioned documentation is coming in the future.
 *
 * OpenAPI spec version: Latest
 * Contact: hi@ory.am
 * Generated by: https://openapi-generator.tech
 */

use hyper;
use std::collections::HashMap;

pub struct Configuration<C: hyper::client::Connect> {
  pub base_path: String,
  pub user_agent: Option<String>,
  pub client: hyper::client::Client<C>,
  pub basic_auth: Option<BasicAuth>,
  pub oauth_access_token: Option<String>,
  pub api_key: Option<ApiKey>,
  // TODO: take an oauth2 token source, similar to the go one
}

pub type BasicAuth = (String, Option<String>);

pub struct ApiKey {
  pub prefix: Option<String>,
  pub key: String,
}

impl<C: hyper::client::Connect> Configuration<C> {
  pub fn new(client: hyper::client::Client<C>) -> Configuration<C> {
    Configuration {
      base_path: "http://localhost".to_owned(),
      user_agent: Some("OpenAPI-Generator/Latest/rust".to_owned()),
      client: client,
      basic_auth: None,
      oauth_access_token: None,
      api_key: None,
    }
  }
}
