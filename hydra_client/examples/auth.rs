extern crate futures;
extern crate hyper;
extern crate hydra_client;
extern crate tokio_core;
extern crate oauth2;

use hyper::Client;
use hyper::client::HttpConnector;
use tokio_core::reactor::Core;
use futures::Future;
use hydra_client::apis::client::APIClient;
use hydra_client::apis::Error;
use hydra_client::apis::configuration;

fn main() {
    let mut core = Core::new().expect("failed to init core");
    let handle = core.handle();

    let mut configuration = configuration::Configuration::new(
        Client::configure()
            .connector(HttpConnector::new(4, &handle))
            .build(&handle),
    );
    configuration.base_path = std::env::var("HYDRA_URL").unwrap().trim_right_matches("/").to_string();
    configuration.basic_auth = Some(("admin".to_owned(), Some("password".to_owned())));

    let apicli = APIClient::new(configuration);
    let work = apicli
        .o_auth2_api()
        .introspect_o_auth2_token("test", "test")
        .then(|r| {
            println!("{:?}", r);
            Ok(()).map_err(|()| ())
        });

    core.run(work).expect("failed to run core");
}
