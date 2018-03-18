extern crate hyper;
extern crate hydra_oauthed_client;
extern crate tokio_core;
extern crate futures;

use hyper::Client;
use futures::future::Future;

use hydra_oauthed_client::HydraClientWrapper;

fn main() {
    let mut core = tokio_core::reactor::Core::new().unwrap();
    let handle = core.handle();
    let client = Client::new(&handle);

    let base_path = std::env::var("HYDRA_URL").unwrap();
    let c = HydraClientWrapper::new(client, base_path.trim_right_matches("/"), "admin".to_owned(), "password".to_owned());

    let work = c.client().warden_api().get_group("users");

    let resp = core.run(work);
    println!("{:?}", resp);
}
