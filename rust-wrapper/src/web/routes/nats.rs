use rocket::{post, serde::json::Json, State};
use rocket_okapi::openapi;
use async_nats::Client;
use serde_json::Value;
use tokio::sync::Mutex;
use std::sync::Arc;

/// **POST /api/nats/event**  
/// Отправляет сообщение в NATS-топик `order-notifications`
#[openapi(tag = "NATS Integration")]
#[post("/nats/event", format = "json", data = "<event>")]
pub async fn publish_nats_event(
    nats_client: &State<Arc<Mutex<Client>>>,
    event: Json<Value>,
) -> Result<Json<String>, Json<String>> {
    let topic = "order-notifications".to_string(); // Явно указываем тип String
    let json_data = serde_json::to_string(&event.into_inner())
        .map_err(|e| Json(format!("Ошибка сериализации: {e}")))?;

    let nats = nats_client.lock().await;

    match nats.publish(topic, json_data.into()).await {
        Ok(_) => Ok(Json("Сообщение отправлено в NATS".to_string())),
        Err(e) => Err(Json(format!("Ошибка отправки в NATS: {e}"))),
    }
}
