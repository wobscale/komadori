use db::users::User;
use diesel;
use diesel::prelude::*;
use std::time::SystemTime;
use db::schema::{groups, users, users_groups};
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Identifiable)]
pub struct Group {
    id: i32,
    pub uuid: Uuid,
    pub name: String,
    pub public: bool,
    pub description: String,
    created_at: SystemTime,
    updated_at: SystemTime,
}

impl Group {
    pub fn from_uuid(conn: &diesel::PgConnection, uuid_: Uuid) -> Result<Self, diesel::result::Error> {
        groups::table.filter(groups::uuid.eq(uuid_)).first(&*conn)
    }

    pub fn owners(&self, conn: &diesel::PgConnection) -> Result<Vec<User>, diesel::result::Error> {
        users::table.left_join(users_groups::table.on(users_groups::group_id.eq(self.id)))
            .select(users::table::all_columns())
            .filter(users_groups::owner.eq(true))
            .load::<User>(&*conn)
    }

    pub fn members(&self, conn: &diesel::PgConnection) -> Result<Vec<User>, diesel::result::Error> {
        users::table.left_join(users_groups::table.on(users_groups::group_id.eq(self.id)))
            .select(users::table::all_columns())
            .load::<User>(&*conn)
    }
}
