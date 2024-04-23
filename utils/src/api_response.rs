use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json
};
use serde_json::json;

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug)]
pub enum ApiError {
    BadRequest,
    Forbidden,
    NotFound(String),
    RequestTimeout,
    InternalServerError,
    InternalServerErrorWithContext(String),
    // NotImplemented,
    // ServiceUnavailable,
    AuthError(AuthError)
}

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Self::BadRequest => (StatusCode::BAD_REQUEST, "Bad request".to_owned()),
            Self::Forbidden => (StatusCode::FORBIDDEN, "Forbidden".to_owned()),
            Self::NotFound(err) => (StatusCode::NOT_FOUND, err),
            Self::RequestTimeout => (StatusCode::REQUEST_TIMEOUT, "Request timeout".to_owned()),
            Self::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_owned()),
            Self::InternalServerErrorWithContext(err) => (StatusCode::INTERNAL_SERVER_ERROR, err),

            Self::AuthError(error_status) => {
                match error_status {
                    AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials".to_owned()),
                    AuthError::MissingCredentials => (StatusCode::UNAUTHORIZED, "Missing credentials".to_owned()),
                    AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error".to_owned()),
                    AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token".to_owned()),
                }
            }
        };

        let body = Json(json!({"error": error_message}));
        (status, body.to_owned()).into_response()
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) =  match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials".to_owned()),
            AuthError::MissingCredentials => (StatusCode::UNAUTHORIZED, "Missing credentials".to_owned()),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error".to_owned()),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token".to_owned()),
        };

        let body = Json(json!({"error": error_message}));
        (status, body.to_owned()).into_response()
    }
}
