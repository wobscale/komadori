use db;
use diesel::prelude::*;
use diesel;
use std::time::{SystemTime, Instant};
use oauth;
use uuid::Uuid;
use schema::{users, groups, users_groups, github_accounts};

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
}

#[derive(Insertable)]
#[table_name = "github_accounts"]
pub struct NewGithubAccount<'a> {
    pub id: i32,
    pub access_token: &'a str,
}


#[derive(Debug, Clone, Queryable, Identifiable)]
pub struct User {
    id: i32,
    pub uuid: Uuid,
    pub username: String,
    pub role: Option<String>,
    pub email: String,
    created_at: SystemTime,
    updated_at: SystemTime,
}

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

#[derive(Debug, Clone, Queryable, Identifiable)]
pub struct GithubAccount {
    pub id: i32,
    user_id: i32,
    access_token: String,
}

#[derive(Debug)]
pub enum GetUserError {
    DbError(diesel::result::Error),
    NoSuchUser,
}

impl<'a> NewUser<'a> {
    pub fn insert_github(self, conn: &diesel::PgConnection, gh: NewGithubAccount) -> Result<User, diesel::result::Error> {
        // TODO: error handling, e.g. detect client vs server errors (such as uniqueness constraints
        // being client, and db conn errs being server)
        let create_res = (&*conn).transaction::<_, diesel::result::Error, _>(|| {
            use diesel;
            use diesel::prelude::*;
            use schema::users::dsl::*;
            use schema::github_accounts;
            let newuser: User = diesel::insert_into(users).values(&self)
                .get_result(&*conn)?;

            diesel::insert_into(github_accounts::table)
                .values((&gh, github_accounts::user_id.eq(newuser.id)))
                .execute(&*conn)?;

            Ok(newuser)
        });
        create_res
    }
}

impl User {
    pub fn from_uuid(conn: &diesel::PgConnection, uuid_: Uuid) -> Result<Self, GetUserError> {
        use diesel::prelude::*;
        use schema::users::dsl::*;
        match users.filter(uuid.eq(uuid_)).limit(1).load::<User>(conn) {
            Ok(u) => match u.first() {
                Some(u) => Ok(u.clone()),
                None => {
                    error!("error getting user {}", uuid_);
                    Err(GetUserError::NoSuchUser)
                }
            },
            Err(e) => {
                error!("error getting user {}", uuid_);
                Err(GetUserError::DbError(e))
            }
        }
    }

    pub fn from_oauth_provider(
        conn: &diesel::PgConnection,
        provider: &oauth::Provider,
        provider_id: &i32,
    ) -> Result<Self, GetUserError> {
        // Compile-check that we can assume github's the only provider
        match provider {
            oauth::Provider::Github => (),
        };

        use diesel::prelude::*;
        use schema::github_accounts;
        use schema::users::dsl::*;
        match {
            let timer = Instant::now();
            let res = users
                .inner_join(github_accounts::table)
                .select(users::all_columns())
                .filter(github_accounts::id.eq(provider_id))
                .limit(1)
                .load::<User>(conn);
            debug!(
                "Partial user to user query took {}",
                (timer.elapsed().as_secs() as f64 + timer.elapsed().subsec_nanos() as f64 * 1e-9)
            );
            res
        } {
            Ok(u) => match u.first() {
                Some(u) => Ok(u.clone()),
                None => Err(GetUserError::NoSuchUser),
            },
            Err(e) => Err(GetUserError::DbError(e)),
        }
    }

    pub fn groups(&self, conn: db::Conn) -> Result<Vec<Group>, String> {
        use diesel::prelude::*;

        groups::table.left_join(users_groups::table.on(users_groups::user_id.eq(self.id)))
            .select(groups::table::all_columns())
            .filter(users_groups::user_id.eq(self.id))
            .load::<Group>(&*conn)
            .map_err(|e| {
                format!("error loading groups: {}", e)
            })
    }

    pub fn add_group(&self, conn: db::Conn, group: Uuid) -> Result<(), diesel::result::Error> {
        let group_id = groups::table
            .select(groups::id)
            .filter(groups::uuid.eq(group))
            .first::<i32>(&*conn)?;

        diesel::insert_into(users_groups::table)
            .values((
                users_groups::user_id.eq(self.id),
                users_groups::group_id.eq(group_id),
            ))
            .execute(&*conn)?;
        Ok(())
    }
}
