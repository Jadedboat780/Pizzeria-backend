pub mod user;

use axum::Json;
use utils::api_response::ApiError;

pub async fn hello_word() -> Json<String> {
    Json("Hello, World!".to_string())
}

pub async fn handler_404() -> ApiError {
    ApiError::NotFound
}
