use crate::models::user::{
    CheckUserByEmail, CheckUserByUsername, CreateUser, UpdateUser, UpdateUserPartial, User,
};
use crate::queries::user::{
    self, insert_user, patch_update_user, put_update_user, select_user_by_email, select_user_by_id,
    select_user_by_username,
};
use crate::AppState;
use api_response::{ApiError, ApiResult};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing, Json, Router,
};
use serde_json::{json, Value};
use utils::{
    encryption::verify_password,
    // jwt::Claims,
};
use uuid::Uuid;

pub async fn router_user(state: AppState) -> Router {
    Router::new()
        .route("/", routing::post(post_user))
        .route(
            "/{id}",
            routing::get(get_user_by_id)
                .put(put_user)
                .patch(patch_user)
                .delete(delete_user),
        )
        .route("/search/email", routing::get(get_user_by_email))
        .route("/search/username", routing::get(get_user_by_username))
        .with_state(state)
}

async fn get_user_by_id(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> ApiResult<Json<User>> {
    let result = select_user_by_id(id, &state.db)
        .await
        .map_err(|err| ApiError::InternalServerError(err.to_string()))?
        .ok_or(ApiError::Forbidden)?;

    Ok(Json(result))
}

async fn get_user_by_email(
    State(state): State<AppState>,
    Json(user_data): Json<CheckUserByEmail>,
) -> ApiResult<Json<Value>> {
    let result = select_user_by_email(&user_data, &state.db)
        .await
        .map_err(|err| ApiError::InternalServerError(err.to_string()))?
        .ok_or(ApiError::Forbidden)?;

    let is_exist = verify_password(&user_data.password, &result.password).await;
    let response = Json(json!({"is_exist": is_exist}));
    Ok(response)
}

async fn get_user_by_username(
    State(state): State<AppState>,
    Json(user_data): Json<CheckUserByUsername>,
) -> ApiResult<Json<Value>> {
    let result = select_user_by_username(&user_data, &state.db)
        .await
        .map_err(|err| ApiError::InternalServerError(err.to_string()))?
        .ok_or(ApiError::Forbidden)?;

    let is_exist = verify_password(&user_data.password, &result.password).await;
    let response = Json(json!({"is_exist": is_exist}));
    Ok(response)
}

async fn post_user(
    State(state): State<AppState>,
    Json(json): Json<CreateUser>,
) -> ApiResult<StatusCode> {
    insert_user(json, &state.db)
        .await
        .map_err(|err| ApiError::InternalServerError(err.to_string()))?;

    Ok(StatusCode::CREATED)
}

async fn put_user(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    Json(update_data): Json<UpdateUser>,
) -> ApiResult<StatusCode> {
    put_update_user(id, update_data, &state.db)
        .await
        .map_err(|err| ApiError::InternalServerError(err.to_string()))?;

    Ok(StatusCode::CREATED)
}

async fn patch_user(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    Json(update_data): Json<UpdateUserPartial>,
) -> ApiResult<StatusCode> {
    patch_update_user(id, update_data, &state.db)
        .await
        .map_err(|err| ApiError::InternalServerError(err.to_string()))?;

    Ok(StatusCode::CREATED)
}

async fn delete_user(Path(id): Path<Uuid>, State(state): State<AppState>) -> ApiResult<StatusCode> {
    user::delete_user(id, &state.db)
        .await
        .map_err(|err| ApiError::InternalServerError(err.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
