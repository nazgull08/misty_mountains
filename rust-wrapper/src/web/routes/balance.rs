use rocket::{post, serde::json::Json, State};
use rocket_okapi::openapi;
use reqwest::Client;
use serde_json::Value;
use sqlx::PgPool;
use crate::crypto::decrypt_secret;
use crate::types::BalanceRequest;
use crate::web::guards::AdminGuard;
use crate::config::Config;

/// **POST /api/balance** 
#[openapi(tag = "Balance Management")]
#[post("/balance", format = "json", data = "<balance_req>")]
pub async fn get_balance_route(
    pool: &State<PgPool>,
    config: &State<Config>,
    _admin: AdminGuard,
    balance_req: Json<BalanceRequest>,
) -> Result<Json<Value>, Json<String>> {
    let user = sqlx::query!(
        "SELECT api_key, encrypted_secret, exchange FROM users WHERE user_telegram_id = $1",
        balance_req.user_telegram_id
    )
    .fetch_one(pool.inner())
    .await
    .map_err(|_| Json("User not found".to_string()))?;

    let real_secret = match decrypt_secret(&user.encrypted_secret, &config.salt_key) {
        Ok(sec) => sec,
        Err(e) => return Err(Json(format!("Decryption error: {e}"))),
    };

    let url = "http://localhost:3000/get_balance";
    let client = Client::new();
    let body = serde_json::json!({
        "exchange": user.exchange,
        "apiKey": user.api_key,
        "secret": real_secret
    });

    let resp = client.post(url).json(&body).send().await
        .map_err(|e| Json(format!("Request error: {:?}", e)))?;

    let json_value: Value = resp.json().await
        .map_err(|e| Json(format!("JSON parse error: {:?}", e)))?;

    Ok(Json(json_value))
}
