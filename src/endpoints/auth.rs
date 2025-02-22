use crate::crud::user as user_crud;
use crate::AppState;
use api_response::{ApiError, ApiResult, AuthError};
use axum::{extract::State, routing, Json, Router};
use jsonwebtoken::{encode, Header};
use jwt::{
    key::KEYS,
    models::{AuthBody, AuthPayload, Claims},
};
use uuid::Uuid;

pub async fn router(state: AppState) -> Router {
    Router::new()
        .route("/", routing::post(authorize))
        .with_state(state)
}

type TokenResult<T> = Result<T, jsonwebtoken::errors::Error>;
fn new_access_token(id: Uuid) -> TokenResult<AuthBody> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: id,
        exp: expiration,
    };

    let token = encode(&Header::default(), &claims, &KEYS.encoding)?;
    Ok(AuthBody::new(token, id))
}

async fn authorize(
    State(state): State<AppState>,
    Json(payload): Json<AuthPayload>,
) -> ApiResult<Json<AuthBody>> {
    if payload.email.is_empty() || payload.password.is_empty() {
        return Err(ApiError::AuthError(AuthError::MissingCredentials));
    }

    let user = user_crud::select_by_email(&payload.email, &state.db)
        .await
        .map_err(|err| ApiError::InternalServerError(err.to_string()))?
        .ok_or(ApiError::AuthError(AuthError::WrongCredentials))?;

    let auth_body =
        new_access_token(user.id).map_err(|_| ApiError::AuthError(AuthError::TokenCreation))?;
    Ok(Json(auth_body))
}
