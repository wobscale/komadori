extern crate rocket;
extern crate komadori;
extern crate serde_json;

use std::env;
use komadori::*;
use rocket::local::*;
use rocket::http::Status;
use rocket::http::Cookie;

fn rocket() -> rocket::Rocket {
    let hydra_conf = {
        let url = must_env("HYDRA_URL");
        let client_id = must_env("HYDRA_CLIENT_ID");
        let client_secret = must_env("HYDRA_CLIENT_SECRET");
        komadori::HydraConfig {
            url: url,
            client_id: client_id,
            client_secret: client_secret,
        }
    };

    let pool = {
        let uri = &must_env("DATABASE_URL");
        komadori::db::db_pool(uri).expect("error connecting to database")
    };

    komadori::rocket(komadori::Config{
        base_url: "".to_string(),
        hydra: hydra_conf,
        github_provider: None,
        local_provider: Some(provider::local::Local::new()),
        pool: pool,
        environment: Environment::Dev,
    })
}

#[test]
fn it_lets_users_login() {
    let rkt = rocket();
    let c = Client::new(rkt).unwrap();

    let not_logged_in = c.get("/user").dispatch();
    assert_eq!(not_logged_in.status(), Status::NotFound);

    // Now use the dev flow to create an account
    let mut req = c.post("/user/create");
    let body = types::CreateUserRequest{
        username: "test_account".to_string(),
        email: "test@email.com".to_string(),
        partial_user: types::PartialUser{
            provider: oauth::Provider::Local,
            provider_id: 1,
            provider_name: "testuser".to_string(),
            access_token: "foo".to_string(),
        }
    };
    req.set_body(serde_json::to_string(&body).unwrap());
    req.add_header(rocket::http::ContentType::JSON);

    let mut resp = req.dispatch();
    assert!(resp.status() == rocket::http::Status::Ok);
    let resp: types::UserResp = serde_json::from_str(&resp.body_string().unwrap()).unwrap();
    assert_eq!(resp.username, "test_account");
    assert_eq!(resp.email, "test@email.com");

    let not_logged_in = {
        let mut req = c.get("/user");
        req.add_header(rocket::http::ContentType::JSON);
        req.dispatch()
    };
    assert!(not_logged_in.status() != rocket::http::Status::Ok);

    let mut req = c.post("/user/auth");
    // local provider insecurely just trusts the 'state' as a side-channel; login with it lets us
    // skip doing a really oauth flow
    let body = provider::ProviderAuthRequest::Local(provider::OauthData{
        code: "ignored".to_string(),
        state: "1 test_account ignored_token".to_string(),
    });
    req.set_body(serde_json::to_string(&body).unwrap());
    req.add_header(rocket::http::ContentType::JSON);
    let mut auth_resp = req.dispatch();
    assert_eq!(auth_resp.status(), rocket::http::Status::Ok);
    assert!(auth_resp.headers().contains("Set-Cookie"));
    let auth_resp_data: types::AuthUserResp = serde_json::from_str(&auth_resp.body_string().unwrap()).unwrap();
    let auth_resp_user = match auth_resp_data {
        types::AuthUserResp::UserResp(user) => {
            assert_eq!(&user.username, "test_account");
            user
        }
        _ => {
            panic!("expected UserResp, got {:?}", auth_resp_data);
        }
    };

    let mut user_resp = {
        let mut req = c.get("/user");
        req = req.cookie(Cookie::parse_encoded(auth_resp.headers().get_one("Set-Cookie").unwrap().to_string()).unwrap());
        req.add_header(rocket::http::ContentType::JSON);
        req.dispatch()
    };
    assert_eq!(user_resp.status(), rocket::http::Status::Ok);
    let user_resp_data: types::UserResp = serde_json::from_str(&user_resp.body_string().unwrap()).unwrap();
    assert_eq!(auth_resp_user, user_resp_data);
}

fn must_env(var: &str) -> String {
    env::var(var).expect(&format!("Environment variable '{}' must be set", var))
}
