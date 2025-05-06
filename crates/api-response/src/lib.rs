use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

#[derive(Debug)]
pub enum ApiError {
    BadRequest,
    Forbidden,
    NotFound,
    RequestTimeout,
    InternalServerError(String),
    NotImplemented,
    AuthError(AuthError),
}

impl AuthError {
    fn to_status_and_message(&self) -> (StatusCode, String) {
        match self {
            AuthError::WrongCredentials => {
                (StatusCode::UNAUTHORIZED, "Wrong credentials".to_owned())
            }
            AuthError::MissingCredentials => {
                (StatusCode::UNAUTHORIZED, "Missing credentials".to_owned())
            }
            AuthError::TokenCreation => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Token creation error".to_owned(),
            ),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token".to_owned()),
        }
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = self.to_status_and_message();
        let body = Json(json!({
            "status": status.as_u16(),
            "error": status.canonical_reason().unwrap_or("Unknown error"),
            "message": error_message
        }));
        (status, body).into_response()
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, err) = match self {
            Self::BadRequest => (StatusCode::BAD_REQUEST, "Bad request".to_owned()),
            Self::Forbidden => (StatusCode::FORBIDDEN, "Forbidden".to_owned()),
            Self::NotFound => (StatusCode::NOT_FOUND, "Recurse not found".to_owned()),
            Self::RequestTimeout => (StatusCode::REQUEST_TIMEOUT, "Request timeout".to_owned()),
            Self::NotImplemented => (StatusCode::NOT_IMPLEMENTED, "Not implemented".to_owned()),
            Self::InternalServerError(err) => {
                tracing::error!("Internal server error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_owned())
            },
            Self::AuthError(error_status) => error_status.to_status_and_message(),
        };

        let body = Json(json!({"error": err}));
        (status, body.to_owned()).into_response()
    }
}
