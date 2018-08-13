use diesel;
use uuid::Uuid;
use models::NewGroup;

fn all_groups() -> Vec<NewGroup> {
    vec![admin_group()]
}

pub fn initialize_groups(db: &diesel::PgConnection) -> Result<(), String> {
    use diesel::prelude::*;
    use schema::groups::dsl::*;

    for g in all_groups() {
        diesel::insert_into(groups)
            .values(&g)
            .on_conflict_do_nothing()
            .execute(db)
            .map_err(|e| {
                format!("error creating group {}: {}", g.name, e)
            })?;
    }

    Ok(())
}

pub fn admin_group() -> NewGroup {
    NewGroup {
        name: "admins".to_string(),
        // arbitrary random value
        uuid: Uuid::parse_str("b249560f-c7c2-463a-872a-79c9841d0139").unwrap(),
        public: true,
        description: r#"Wobscale Admins
    This group contains administrators who have far-reaching permissions.
    For more information, contact admin@wobscale.website"#.to_string(),
    }
}
