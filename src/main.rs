extern crate chrono;
extern crate fern;
#[macro_use]
extern crate log;

extern crate komadori;

use komadori::*;
use std::env;


fn main() {
    let env = {
        let var = env::var("ENVIRONMENT").unwrap_or("".to_owned());
        match var.as_str() {
            "dev" => Environment::Dev,
            "prod" => Environment::Prod,
            _ => panic!("ENVIRONMENT must be set to 'dev' or 'prod'"),
        }
    };

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}]\t[{}]\t {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Warn)
        .level_for("rocket", log::LevelFilter::Info)
        .level_for("komadori", {
            if env == Environment::Dev {
                log::LevelFilter::Debug
            } else {
                log::LevelFilter::Warn
            }
        })
        .chain(std::io::stdout())
        .apply()
        .unwrap();

    info!("Running with environmeng: {:?}", env);

    let base_url = if env == Environment::Dev {
        env::var("BASE_URL").unwrap_or("http://127.0.0.1:8000".to_owned())
    } else {
        must_env("BASE_URL")
    };

    let hydra_conf = {
        let url = must_env("HYDRA_URL");
        let client_id = must_env("HYDRA_CLIENT_ID");
        let client_secret = must_env("HYDRA_CLIENT_SECRET");
        HydraConfig {
            url: url,
            client_id: client_id,
            client_secret: client_secret,
        }
    };

    let provider = {
        let client_id = env::var("GITHUB_CLIENT_ID").expect("GITHUB_CLIENT_ID must be set");
        let client_secret = env::var("GITHUB_SECRET_KEY").expect("GITHUB_SECRET_KEY must be set");
        OauthProviderConfig{
            provider: oauth::Provider::Github,
            client_id: client_id,
            client_secret: client_secret,
        }
    };

    let pool = {
        let uri = &must_env("DATABASE_URL");
        db::db_pool(uri).expect("error connecting to database")
    };

    rocket(komadori::Config{
        environment: env,
        base_url: base_url,
        hydra: hydra_conf,
        oauth: vec![provider],
        pool: pool,
    })
    .launch();
}

fn must_env(var: &str) -> String {
    env::var(var).expect(&format!("Environment variable '{}' must be set", var))
}
