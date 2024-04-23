use axum::Json;
use utils::{
    jwt::{encode_token, auth_models::{AuthPayload, AuthBody}},
    api_response::{ApiResult, ApiError, AuthError}
};

pub async fn authorize(Json(payload): Json<AuthPayload>) -> ApiResult<Json<AuthBody>> {
    if payload.client_id.is_empty() || payload.client_secret.is_empty() {
        return Err(ApiError::AuthError(AuthError::MissingCredentials));
    }

    if payload.client_secret != "super_secret_key" {
        return Err(ApiError::AuthError(AuthError::InvalidToken));
    }

    let token = encode_token(payload.client_id).map_err(|_| ApiError::AuthError(AuthError::InvalidToken))?;
    Ok(Json(AuthBody::new(token)))
}
