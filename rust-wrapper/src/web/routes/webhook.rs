use rocket::{post, serde::json::Json, State};
use rocket_okapi::openapi;
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;
use async_nats::Client;
use tokio::sync::Mutex;
use std::sync::Arc;

use crate::crypto::decrypt_secret;
use crate::config::Config;
use crate::types::TradingViewSignal;

/// **POST /webhook/<strategy_uid>**  
#[openapi(tag = "Webhook")]
#[post("/webhook/<strategy_uid>", format = "json", data = "<payload>")]
pub async fn webhook_handler(
    pool: &State<PgPool>,
    config: &State<Config>,
    nats_client: &State<Arc<Mutex<Client>>>,
    strategy_uid: Uuid,
    payload: Json<TradingViewSignal>,
) -> Result<Json<String>, Json<String>> {
    // 1. Находим стратегию и её пользователя
    let strategy = sqlx::query!(
        "SELECT strategies.id, users.api_key, users.encrypted_secret, users.exchange
         FROM strategies
         JOIN users ON strategies.user_id = users.id
         WHERE strategies.id = $1",
        strategy_uid
    )
    .fetch_one(pool.inner())
    .await
    .map_err(|_| Json("Strategy not found".to_string()))?;

    // 2. Расшифровываем secret_key
    let real_secret = decrypt_secret(&strategy.encrypted_secret, &config.salt_key)
        .map_err(|e| Json(format!("Decryption error: {e}")))?;

    // 3. Формируем сообщение для NATS
    let order_data = json!({
        "exchange": strategy.exchange,
        "apiKey": strategy.api_key,
        "secret": real_secret,
        "order_id": payload.id,
        "side": payload.signal.to_lowercase(),
        "symbol": payload.ticker,
        "amount": payload.contracts.parse::<f64>().unwrap_or(0.0),
        "price": if payload.order_price == "market" { None } else { Some(payload.order_price.parse::<f64>().unwrap_or(0.0)) },
        "strategyUid": strategy_uid,
        "title": payload.title
    });

    let topic = "trading-signals".to_string();
    let nats = nats_client.lock().await;
    nats.publish(topic, serde_json::to_string(&order_data).expect("invalid nats topic order data").into())
        .await
        .map_err(|e| Json(format!("Error sending to NATS: {e}")))?;

    Ok(Json("Webhook received and published to NATS".to_string()))
}
