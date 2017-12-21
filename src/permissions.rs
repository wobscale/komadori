use hydra::client::Client;

pub const USER_GROUP: &str = "users";
pub const DEV_GROUP: &str = "wobdevs";

fn all_groups() -> Vec<&'static str> {
    vec![USER_GROUP, DEV_GROUP]
}

pub fn initialize_groups(hydra: &Client) -> Result<(), String> {
    // This might be racy, but fortunately we only run one of these at a time anyways
    for group in all_groups() {
        match hydra.warden_group_get(group) {
            Ok(_) => {
                return Ok(());
            }
            Err(e) => {
                debug!(
                    "warden: got error {}, assuming group {} doesn't exist",
                    e, group
                );
            }
        };

        hydra.warden_group_create(group, Vec::new())?;
    }
    Ok(())
}
