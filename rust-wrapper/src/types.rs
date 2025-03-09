use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BalanceRequest {
    pub user_telegram_id: i64,
    pub exchange: String,
}

/// **Запрос на регистрацию пользователя**
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterUserRequest {
    pub user_telegram_id: i64,
    pub api_key: String,
    pub secret_key: String,
    pub exchange: String,
}

/// **Ответ на регистрацию пользователя**
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterUserResponse {
    pub user_uid: Uuid,
}

/// **Запрос на обновление пользователя**
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserRequest {
    pub api_key: String,
    pub secret_key: Option<String>,
    pub exchange: String,
}

/// **Структура пользователя**
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Uuid,
    pub user_telegram_id: i64,
    pub exchange: String,
}

/// **Запрос на создание стратегии**
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateStrategyRequest {
    pub user_uid: Uuid,
    pub strategy_name: String,
}

/// **Ответ на создание стратегии**
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateStrategyResponse {
    pub webhook: String,
    pub strategy_uid: Uuid,
}

/// **Запрос на включение/выключение стратегий**
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ToggleStrategiesRequest {
    pub strategy_uids: Vec<Uuid>,
}

/// **Структура стратегии**
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Strategy {
    pub strategy_uid: Uuid,
    pub strategy_name: String,
    pub enabled: bool,
}

/// **Ответ на получение списка стратегий**
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct StrategiesResponse {
    pub personal: Vec<Strategy>,
    pub other: Vec<Strategy>,
}

/// **Запрос от TradingView**
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct TradingViewSignal {
    pub id: String,
    pub signal: String,  // "buy" / "sell"
    pub contracts: String, // Количество контрактов
    pub ticker: String, // "NEAR/USDT"
    pub order_price: String, // Цена ордера (или "market")
    pub deposit_pct_limit: String, // Лимит депозита в процентах
    pub order_type: String, // "spot" или другое
    pub title: String, // Доп. информация
    pub sl_percentage: String, // Стоп-лосс
}
