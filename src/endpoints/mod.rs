pub mod auth;
pub mod pizza;
pub mod user;

use api_response::ApiError;
use axum::Json;
use serde_json::json;

pub async fn ping() -> Json<serde_json::Value> {
    Json(json!({"ping": "pong!"}))
}

pub async fn handler_404() -> ApiError {
    ApiError::NotFound
}
