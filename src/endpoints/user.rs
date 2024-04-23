use axum::{extract::{Path, State}, Json, Router, routing, http::StatusCode};
use serde_json::{json, Value};
use utils::{
    api_response::{ApiResult, ApiError},
    encryption::{hash_password, verify_password},
    jwt::Claims
};
use crate::AppState;
use crate::models::user::{CreateUser, GetUserByEmail, GetUserByUsername, UpdateUser};
use crate::queries::user::{create_user, delete_user, get_user_by_email, get_user_by_username, update_user};

pub async fn router_user(state: AppState) -> Router{
    Router::new()
        .route("/search/email", routing::get(search_user_by_email))
        .route("/search/username", routing::get(search_user_by_username))
        .route("/", routing::post(post_user))
        .route("/:id", routing::patch(patch_user).delete(del_user))
        .with_state(state)
}


async fn search_user_by_email(
    State(state): State<AppState>,
    Json(user_data): Json<GetUserByEmail>
) -> ApiResult<Json<Value>> {
    let result = get_user_by_email(&user_data, &state.db)
        .await
        .map_err(|_| ApiError::InternalServerError)?
        .ok_or(ApiError::Forbidden)?;

    let is_exist = verify_password(&user_data.password, &result.password).await;
    let response = Json(json!({"is_exist": is_exist}));
    Ok(response)
}

async fn search_user_by_username(
    State(state): State<AppState>,
    Json(user_data): Json<GetUserByUsername>
) -> ApiResult<Json<Value>> {
    let result = get_user_by_username(&user_data, &state.db)
        .await
        .map_err(|_| ApiError::InternalServerError)?
        .ok_or(ApiError::Forbidden)?;

    let is_exist = verify_password(&user_data.password, &result.password).await;
    let response = Json(json!({"is_exist": is_exist}));
    Ok(response)
}

async fn post_user(
    State(state): State<AppState>,
    Json(json): Json<CreateUser>
) -> ApiResult<StatusCode>  {
    create_user(json, &state.db)
        .await
        .map_err(|_| ApiError::InternalServerError)?;

    Ok(StatusCode::CREATED)
}

async fn patch_user(
    Path(id): Path<i32>,
    State(state): State<AppState>,
    Json(json): Json<UpdateUser>
) -> ApiResult<StatusCode> {
    update_user(id, json, &state.db)
        .await
        .map_err(|_| ApiError::InternalServerError)?;

    Ok(StatusCode::NO_CONTENT)
}

async fn del_user(
    Path(id): Path<i32>,
    State(state): State<AppState>
) -> ApiResult<StatusCode> {
    delete_user(id, &state.db)
        .await
        .map_err(|_| ApiError::InternalServerError)?;

    Ok(StatusCode::NO_CONTENT)
}