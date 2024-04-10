use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::{json, Value};

pub type ApiResult = Result<ApiResponse, ApiError>;

#[derive(Debug)]
pub enum ApiResponse {
    OK,
    OKWithJSON(Json<Value>),
    Created,
}

#[derive(Debug)]
pub enum ApiError {
    BadRequest,
    Forbidden,
    // Unauthorised,
    InternalServerError,
    NotFound,
}

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        match self {
            Self::OK => (StatusCode::OK).into_response(),
            Self::OKWithJSON(json) => (StatusCode::OK, json).into_response(),
            Self::Created => (StatusCode::CREATED).into_response(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Self::BadRequest => (StatusCode::BAD_REQUEST, "hello"),
            Self::Forbidden => (StatusCode::FORBIDDEN, "hello"),
            Self::InternalServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            Self::NotFound => (StatusCode::NOT_FOUND, "Page not found"),
        };

        let body = Json(json!({"error": error_message}));
        (status, body).into_response()
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };

        let body = Json(json!({"error": error_message}));
        (status, body).into_response()
    }
}
