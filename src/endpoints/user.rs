use std::result;
use crate::requests::user::{CreateUser, GetUserByEmail, GetUserByUsername, UpdateUser};
use crate::user_query::{create_user, get_user_by_email, get_user_by_username, update_user};
use crate::AppState;
use axum::extract::{Path, State};
use axum::Json;
use serde_json::json;
use utils::api_response::{ApiError, ApiResponse, ApiResult};
use utils::encryption::{hash_password, verify_password};
use utils::jwt::Claims;

pub async fn search_user_by_email(
    _claims: Claims,
    State(state): State<AppState>,
    Json(user_data): Json<GetUserByEmail>
) -> ApiResult {
    let result = get_user_by_email(&user_data, &state.db)
        .await
        .map_err(|_| ApiError::InternalServerError)?
        .ok_or(ApiError::Forbidden)?;

    let is_exist = verify_password(&user_data.password, &result.password).await;
    let response = Json(json!({"is_exist": is_exist}));
    Ok(ApiResponse::OKWithJSON(response))
}

pub async fn search_user_by_username(
    _claims: Claims,
    State(state): State<AppState>,
    Json(user_data): Json<GetUserByUsername>
) -> ApiResult {
    let result = get_user_by_username(&user_data, &state.db)
        .await
        .map_err(|_| ApiError::InternalServerError)?
        .ok_or(ApiError::Forbidden)?;

    let is_exist = verify_password(&user_data.password, &result.password).await;
    let response = Json(json!({"is_exist": is_exist}));
    Ok(ApiResponse::OKWithJSON(response))
}

pub async fn new_user(
    _claims: Claims,
    State(state): State<AppState>,
    Json(json): Json<CreateUser>,
) -> ApiResult {
    create_user(json, &state.db)
        .await
        .map_err(|_| ApiError::InternalServerError)?;

    Ok(ApiResponse::OK)
}

pub async fn patch_user(
    _claims: Claims,
    Path(id): Path<i32>,
    State(state): State<AppState>,
    Json(json): Json<UpdateUser>,
) -> ApiResult {
    update_user(id, json, &state.db)
        .await
        .map_err(|_| ApiError::InternalServerError)?;

    Ok(ApiResponse::OK)
}
