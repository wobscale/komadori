use hydra;
use hydra_client;
use futures;
use tokio_core;
use futures::future::Future;

pub const USER_GROUP: &str = "users";
pub const DEV_GROUP: &str = "wobdevs";
pub const ADMIN_GROUP: &str = "admins";

fn all_groups() -> Vec<&'static str> {
    vec![USER_GROUP, DEV_GROUP, ADMIN_GROUP]
}

pub fn initialize_groups(hydra: &hydra::client::ClientBuilder) -> Result<(), String> {
    // This self-contained core is rather hacky, but it's quite tricky to get lifetimes right with
    // returning a future from this function as it was written, and having the core in main doesn't
    // gain us anything until rocket converts to async... so for now, this is the easiest way to
    // make it borrowck even if it's icky code.
    let mut core = tokio_core::reactor::Core::new().unwrap();
    let wrapper = hydra.build(&core.handle());
    let client = wrapper.client();

    // This might be racy, but fortunately we only run one of these at a time anyways
    let group_futures = all_groups().into_iter().map(|group| {
        let f = client.warden_api().get_group(group);
        (f, group)
    });


    let group_res = group_futures.map(|(future, group)| {
        let client2 = wrapper.client();
        future.then(move |res| {
            match res {
                Err(e) => {
                    debug!(
                        "warden: got error {:?}, assuming group doesn't exist",
                        e,
                    );
                    let f: Box<futures::Future<Error = String, Item = ()>> = Box::new(client2.warden_api().create_group(hydra_client::models::Group::new().with_id(group.clone().to_owned())).map(|g| {
                        info!("created group: {}", g.id().unwrap());
                        ()
                    }).map_err(|e| format!("error creating group: {:?}", e)));
                    f
                },
                Ok(_) => {
                    let f: Box<futures::Future<Error = String, Item = ()>> = Box::new(futures::future::ok(()));
                    f
                }
            }
        })
        .map_err(|e| {
            format!("error creating group: {:?}", e)
        })
    }).collect::<Vec<_>>();

    core.run(futures::future::join_all(group_res)).map(|_| ())
}
