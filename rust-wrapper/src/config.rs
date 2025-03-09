use dotenv::dotenv;
use std::env;

pub struct Config {
    pub domain: String,
    pub admin_token: String,
    pub salt_key: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();
        let domain = env::var("DOMAIN").expect("DOMAIN must be set");
        let admin_token = env::var("ADMIN_TOKEN").expect("ADMIN_TOKEN must be set");
        let salt_key = env::var("SALT_KEY").expect("SALT_KEY must be set");

        Config { domain, admin_token, salt_key}
    }
}
