use axum::Json;
use utils::{
    encode_token,
    jwt::{auth_body::AuthBody, auth_payload::AuthPayload},
    api_response::AuthError
};

pub async fn authorize(Json(payload): Json<AuthPayload>) -> Result<Json<AuthBody>, AuthError> {
    if payload.client_id.is_empty() || payload.client_secret.is_empty() {
        return Err(AuthError::MissingCredentials);
    }

    if payload.client_secret != "super_secret_key" {
        return Err(AuthError::InvalidToken);
    }

    let token = encode_token(payload.client_id).map_err(|_| AuthError::TokenCreation)?;
    Ok(Json(AuthBody::new(token)))
}
