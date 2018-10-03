use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  json_web_key_api: Box<::apis::JsonWebKeyApi>,
  o_auth2_api: Box<::apis::OAuth2Api>,
  policy_api: Box<::apis::PolicyApi>,
  warden_api: Box<::apis::WardenApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      json_web_key_api: Box::new(::apis::JsonWebKeyApiClient::new(rc.clone())),
      o_auth2_api: Box::new(::apis::OAuth2ApiClient::new(rc.clone())),
      policy_api: Box::new(::apis::PolicyApiClient::new(rc.clone())),
      warden_api: Box::new(::apis::WardenApiClient::new(rc.clone())),
    }
  }

  pub fn json_web_key_api(&self) -> &::apis::JsonWebKeyApi{
    self.json_web_key_api.as_ref()
  }

  pub fn o_auth2_api(&self) -> &::apis::OAuth2Api{
    self.o_auth2_api.as_ref()
  }

  pub fn policy_api(&self) -> &::apis::PolicyApi{
    self.policy_api.as_ref()
  }

  pub fn warden_api(&self) -> &::apis::WardenApi{
    self.warden_api.as_ref()
  }


}
