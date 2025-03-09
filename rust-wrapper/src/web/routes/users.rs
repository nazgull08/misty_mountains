use rocket::delete;
use rocket::{get, post, put, serde::json::Json, State};
use rocket_okapi::openapi;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::crypto::encrypt_secret;
use crate::types::{RegisterUserRequest, RegisterUserResponse, UpdateUserRequest, User};
use crate::web::guards::AdminGuard;
use crate::config::Config;

/// **POST /api/user** — Регистрация пользователя**
#[openapi(tag = "User Management")]
#[post("/user", format = "json", data = "<user_data>")]
pub async fn register_user(
    pool: &State<PgPool>,
    config: &State<Config>,
    _admin: AdminGuard,
    user_data: Json<RegisterUserRequest>,
) -> Result<Json<RegisterUserResponse>, Json<String>> {
    let user_uid = Uuid::new_v4();

    let encrypted_secret = encrypt_secret(&user_data.secret_key, &config.salt_key)
        .map_err(|e| Json(format!("Encryption error: {e}")))?;

    sqlx::query!(
        "INSERT INTO users (id, user_telegram_id, api_key, encrypted_secret, exchange) 
         VALUES ($1, $2, $3, $4, $5)",
        user_uid,
        user_data.user_telegram_id,
        user_data.api_key,
        encrypted_secret,
        user_data.exchange
    )
    .execute(pool.inner())
    .await
    .map_err(|e| Json(format!("Database error: {:?}", e)))?;

    Ok(Json(RegisterUserResponse { user_uid }))
}

/// **GET /api/user/<user_uid>** — Получение информации о пользователе**
#[openapi(tag = "User Management")]
#[get("/user/<user_uid>")]
pub async fn get_user(
    pool: &State<PgPool>,
    _admin: AdminGuard,
    user_uid: Uuid,
) -> Result<Json<User>, Json<String>> {
    let result = sqlx::query!(
        "SELECT id, user_telegram_id, exchange FROM users WHERE id = $1",
        user_uid
    )
    .fetch_one(pool.inner())
    .await
    .map_err(|_| Json("User not found".to_string()))?;

    Ok(Json(User {
        id: result.id,
        user_telegram_id: result.user_telegram_id,
        exchange: result.exchange,
    }))
}

/// **GET /api/users** — Получение всех пользователей**
#[openapi(tag = "User Management")]
#[get("/users")]
pub async fn get_all_users(
    pool: &State<PgPool>,
    _admin: AdminGuard,
) -> Result<Json<Vec<User>>, Json<String>> {
    let result = sqlx::query!(
        "SELECT id, user_telegram_id, exchange FROM users"
    )
    .fetch_all(pool.inner())
    .await
    .map_err(|_| Json("Failed to fetch users".to_string()))?;

    let users: Vec<User> = result
        .iter()
        .map(|user| User {
            id: user.id,
            user_telegram_id: user.user_telegram_id,
            exchange: user.exchange.clone(),
        })
        .collect();

    Ok(Json(users))
}

/// **PUT /api/user/<user_uid>** — Обновление пользователя**
#[openapi(tag = "User Management")]
#[put("/user/<user_uid>", format = "json", data = "<update_data>")]
pub async fn update_user(
    pool: &State<PgPool>,
    config: &State<Config>,
    _admin: AdminGuard,
    user_uid: Uuid,
    update_data: Json<UpdateUserRequest>,
) -> Result<Json<String>, Json<String>> {
    let encrypted_secret = if let Some(secret) = &update_data.secret_key {
        Some(encrypt_secret(secret, &config.salt_key).map_err(|e| Json(format!("Encryption error: {e}")))?)
    } else {
        None
    };

    let mut tx = pool.inner().begin().await.map_err(|e| Json(format!("Transaction error: {e}")))?;

    sqlx::query!(
        "UPDATE users 
         SET api_key = $1, 
             encrypted_secret = COALESCE($2, encrypted_secret),
             exchange = $3
         WHERE id = $4",
        update_data.api_key,
        encrypted_secret,
        update_data.exchange,
        user_uid
    )
    .execute(&mut *tx)
    .await
    .map_err(|e| Json(format!("Database error: {:?}", e)))?;

    tx.commit().await.map_err(|e| Json(format!("Commit error: {e}")))?;

    Ok(Json("User updated successfully".to_string()))
}



/// **DELETE /api/user/{userUid}** — Удаление пользователя и всех его стратегий
#[openapi(tag = "User Management")]
#[delete("/user/<user_uid>")]
pub async fn delete_user(
    pool: &State<PgPool>,
    _admin: AdminGuard,
    user_uid: Uuid,
) -> Result<Json<String>, Json<String>> {
    let deleted = sqlx::query!("DELETE FROM users WHERE id = $1", user_uid)
        .execute(pool.inner())
        .await
        .map_err(|e| Json(format!("Database error: {:?}", e)))?
        .rows_affected();

    if deleted == 0 {
        return Err(Json("User not found".to_string()));
    }

    Ok(Json("User deleted successfully".to_string()))
}
