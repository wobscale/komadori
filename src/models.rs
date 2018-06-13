use schema::*;
use uuid::Uuid;

#[derive(Insertable)]
#[table_name = "groups"]
pub struct NewGroup {
    pub name: String,
    pub uuid: Uuid,
    pub public: bool,
    pub description: String,
}


