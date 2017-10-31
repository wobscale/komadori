#[derive(Insertable)]
#[table_name="users"]
struct NewUser<'a> {
	username: &'a str,
	display_name: Option<&'a str>,
	email: Option<&'a str>,
};


#[derive(Insertable)]
#[table_name="oauth_connections"]
struct NewOauthConn<'a> {
	id: i32,
	user_id: i32,
	connector: &'a str,
}
