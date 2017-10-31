#[derive(Insertable)]
#[table_name="users"]
struct NewUser<'a> {
	username: &'a str,
	display_name: Option<&'a str>,
	email: Option<&'a str>,
};

#[derive(Insertable)]
#[table_name="github_accounts"]
struct NewGithubAccount<'a> {
	id: i64,
	user_id: i64,
}
