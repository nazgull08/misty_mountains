pub mod balance;
pub mod nats;
pub mod strategies;
pub mod users;
pub mod webhook;

use rocket::Route;
use rocket_okapi::{openapi_get_routes, swagger_ui::SwaggerUIConfig};

pub fn get_routes() -> Vec<Route> {
    openapi_get_routes![
        // Balance
        balance::get_balance_route,

        // Users 
        users::register_user,
        users::update_user,
        users::get_user,
        users::get_all_users,
        users::delete_user,

        // NATS
        nats::publish_nats_event,

        // Webhook
        webhook::webhook_handler,

        // Strategies 
        strategies::create_strategy,
        strategies::delete_strategy,
        strategies::get_strategy,
        strategies::get_strategies,
        strategies::update_strategy, 
        strategies::toggle_strategies
    ]
}

pub fn get_docs() -> SwaggerUIConfig {
    SwaggerUIConfig {
        url: "/api/openapi.json".to_string(),
        ..Default::default()
    }
}
