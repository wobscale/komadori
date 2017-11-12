use schema::*;

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
    pub user_id: i32,
    pub access_token: &'a str,
}
