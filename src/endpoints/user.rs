use axum::{extract::{Path, State}, Json, Router, routing, http::StatusCode};
use serde_json::{json, Value};
use utils::{
    api_response::{ApiResult, ApiError},
    encryption::verify_password,
    jwt::Claims,
};
use crate::AppState;
use crate::models::user::{User, CheckUserByEmail, CheckUserByUsername, CreateUser, PutUser, PatchUser};
use crate::queries::user::{self, select_user_by_email, insert_user, select_user_by_username, patch_update_user, select_user_by_id, put_update_user};

pub async fn router_user(state: AppState) -> Router {
    Router::new()
        .route("/", routing::post(post_user))
        .route("/:id", routing::get(get_user_by_id).put(put_user).patch(patch_user).delete(delete_user))
        .route("/search/email", routing::get(get_user_by_email))
        .route("/search/username", routing::get(get_user_by_username))
        .with_state(state)
}

async fn get_user_by_id(
    Path(id): Path<i32>,
    State(state): State<AppState>
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
    Path(id): Path<i32>,
    State(state): State<AppState>,
    Json(update_data): Json<PutUser>,
) -> ApiResult<StatusCode> {
    put_update_user(id, update_data, &state.db)
        .await
        .map_err(|err| ApiError::InternalServerError(err.to_string()))?;

    Ok(StatusCode::CREATED)
}

async fn patch_user(
    Path(id): Path<i32>,
    State(state): State<AppState>,
    Json(update_data): Json<PatchUser>
) -> ApiResult<StatusCode> {
    patch_update_user(id, update_data, &state.db)
        .await
        .map_err(|err| ApiError::InternalServerError(err.to_string()))?;

    Ok(StatusCode::CREATED)
}

async fn delete_user(
    Path(id): Path<i32>,
    State(state): State<AppState>
) -> ApiResult<StatusCode> {
    user::delete_user(id, &state.db)
        .await
        .map_err(|err| ApiError::InternalServerError(err.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}