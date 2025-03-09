use rocket::{get, post, put, delete, serde::json::Json, State};
use rocket_okapi::openapi;
use sqlx::PgPool;
use uuid::Uuid;

use crate::types::{CreateStrategyRequest, CreateStrategyResponse, ToggleStrategiesRequest, Strategy, StrategiesResponse};
use crate::web::guards::AdminGuard;
use crate::config::Config;

/// **POST /api/strategy** — Создание стратегии
#[openapi(tag = "Strategy Management")]
#[post("/strategy", format = "json", data = "<strategy_data>")]
pub async fn create_strategy(
    pool: &State<PgPool>,
    config: &State<Config>,
    _admin: AdminGuard,
    strategy_data: Json<CreateStrategyRequest>,
) -> Result<Json<CreateStrategyResponse>, Json<String>> {
    let strategy_uid = Uuid::new_v4();

    sqlx::query!(
        "INSERT INTO strategies (id, user_id, strategy_name) VALUES ($1, $2, $3)",
        strategy_uid,
        strategy_data.user_uid,
        strategy_data.strategy_name
    )
    .execute(pool.inner())
    .await
    .map_err(|e| Json(format!("Database error: {:?}", e)))?;

    let webhook = format!("{}/webhook/{}", config.domain, strategy_uid);

    Ok(Json(CreateStrategyResponse { webhook, strategy_uid }))
}

/// **DELETE /api/strategy/{strategyUid}** — Удаление стратегии
#[openapi(tag = "Strategy Management")]
#[delete("/strategy/<strategy_uid>")]
pub async fn delete_strategy(
    pool: &State<PgPool>,
    _admin: AdminGuard,
    strategy_uid: Uuid,
) -> Result<Json<String>, Json<String>> {
    let deleted = sqlx::query!("DELETE FROM strategies WHERE id = $1", strategy_uid)
        .execute(pool.inner())
        .await
        .map_err(|e| Json(format!("Database error: {:?}", e)))?
        .rows_affected();

    if deleted == 0 {
        return Err(Json("Strategy not found".to_string()));
    }

    Ok(Json("Strategy deleted successfully".to_string()))
}

/// **GET /api/strategies?userUid=...** — Получение списка стратегий
#[openapi(tag = "Strategy Management")]
#[get("/strategies?<user_uid>")]
pub async fn get_strategies(
    pool: &State<PgPool>,
    _admin: AdminGuard,
    user_uid: Uuid,
) -> Result<Json<StrategiesResponse>, Json<String>> {
    let result = sqlx::query!(
        "SELECT id, strategy_name, enabled FROM strategies WHERE user_id = $1",
        user_uid
    )
    .fetch_all(pool.inner())
    .await
    .map_err(|_| Json("Failed to fetch strategies".to_string()))?;

    let mut personal = Vec::new();
    let mut other = Vec::new();

    for row in result {
        let strategy = Strategy {
            strategy_uid: row.id,
            strategy_name: row.strategy_name,
            enabled: row.enabled,
        };

        if strategy.enabled {
            personal.push(strategy);
        } else {
            other.push(strategy);
        }
    }

    Ok(Json(StrategiesResponse { personal, other }))
}

/// **POST /api/strategies/{enable/disable}?userUid=...** — Включение/выключение стратегий
#[openapi(tag = "Strategy Management")]
#[post("/strategies/<action>?<user_uid>", format = "json", data = "<toggle_request>")]
pub async fn toggle_strategies(
    pool: &State<PgPool>,
    _admin: AdminGuard,
    action: &str,
    user_uid: Uuid,
    toggle_request: Json<ToggleStrategiesRequest>,
) -> Result<Json<String>, Json<String>> {
    let enable = match action {
        "enable" => true,
        "disable" => false,
        _ => return Err(Json("Invalid action. Use 'enable' or 'disable'.".to_string())),
    };

    let strategy_uids = &toggle_request.strategy_uids;

    if strategy_uids.is_empty() {
        return Err(Json("No strategies provided".to_string()));
    }

    let mut tx = pool.inner().begin().await.map_err(|e| Json(format!("Transaction error: {e}")))?;

    for strategy_uid in strategy_uids {
        sqlx::query!(
            "UPDATE strategies SET enabled = $1 WHERE id = $2 AND user_id = $3",
            enable,
            strategy_uid,
            user_uid
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| Json(format!("Database error: {:?}", e)))?;
    }

    tx.commit().await.map_err(|e| Json(format!("Commit error: {e}")))?;

    Ok(Json(format!(
        "Strategies successfully {}d",
        if enable { "enable" } else { "disable" }
    )))
}


/// **PUT /api/strategy/{strategyUid}** 
#[openapi(tag = "Strategy Management")]
#[put("/strategy/<strategy_uid>", format = "json", data = "<update_data>")]
pub async fn update_strategy(
    pool: &State<PgPool>,
    _admin: AdminGuard,
    strategy_uid: Uuid,
    update_data: Json<CreateStrategyRequest>,
) -> Result<Json<String>, Json<String>> {
    let updated = sqlx::query!(
        "UPDATE strategies SET strategy_name = $1 WHERE id = $2",
        update_data.strategy_name,
        strategy_uid
    )
    .execute(pool.inner())
    .await
    .map_err(|e| Json(format!("Database error: {:?}", e)))?
    .rows_affected();

    if updated == 0 {
        return Err(Json("Strategy not found".to_string()));
    }

    Ok(Json("Strategy name updated successfully".to_string()))
}


/// **GET /api/strategy/{strategyUid}** — Получение информации о стратегии
#[openapi(tag = "Strategy Management")]
#[get("/strategy/<strategy_uid>")]
pub async fn get_strategy(
    pool: &State<PgPool>,
    _admin: AdminGuard,
    strategy_uid: Uuid,
) -> Result<Json<Strategy>, Json<String>> {
    let strategy = sqlx::query!(
        "SELECT id, strategy_name, enabled FROM strategies WHERE id = $1",
        strategy_uid
    )
    .fetch_one(pool.inner())
    .await
    .map_err(|_| Json("Strategy not found".to_string()))?;

    Ok(Json(Strategy {
        strategy_uid: strategy.id,
        strategy_name: strategy.strategy_name,
        enabled: strategy.enabled,
    }))
}
