use std::iter::Map;
use std::sync::Arc;

use async_nats::Client;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Build, Config, Rocket};
use rocket_okapi::okapi::openapi3::SecurityScheme;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use sqlx::PgPool;
use tokio::sync::Mutex;
use crate::config::Config as AppConfig;
use crate::web::routes::{get_routes, get_docs};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "CORS Middleware",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _: &'r rocket::Request<'_>, res: &mut rocket::Response<'r>) {
        res.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        res.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "GET, POST, PUT, DELETE, OPTIONS",
        ));
        res.set_header(Header::new(
            "Access-Control-Allow-Headers",
            "Content-Type, Authorization",
        ));
    }
}

pub async fn rocket(port: u16, pool: PgPool, nats: Arc<Mutex<Client>>) -> Rocket<Build> {
    let config = Config {
        address: "0.0.0.0".parse().unwrap(),
        port,
        ..Config::default()
    };

    let app_config = AppConfig::from_env();

    rocket::custom(config)
        .manage(pool)
        .manage(app_config) // Передаём конфиг
        .manage(nats)
        .mount("/api", get_routes())
        .mount("/swagger", make_swagger_ui(&get_docs()))
        .attach(CORS)
}

