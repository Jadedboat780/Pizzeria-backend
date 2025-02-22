use crate::crud::user as user_crud;
use crate::models::user::{CreateUser, GetUserByEmail, UpdateUser, UpdateUserPartial, User};
use crate::AppState;
use api_response::{ApiError, ApiResult};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing, Json, Router,
};
use utils::encryption::verify_password;
use uuid::Uuid;

pub async fn router(state: AppState) -> Router {
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
        .with_state(state)
}

async fn get_user_by_id(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> ApiResult<Json<User>> {
    let user = user_crud::select_by_id(id, &state.db)
        .await
        .map_err(|err| ApiError::InternalServerError(err.to_string()))?
        .ok_or(ApiError::Forbidden)?;

    Ok(Json(user))
}

async fn get_user_by_email(
    State(state): State<AppState>,
    Json(user_data): Json<GetUserByEmail>,
) -> ApiResult<Json<User>> {
    let user = user_crud::select_by_email(&user_data.email, &state.db)
        .await
        .map_err(|err| ApiError::InternalServerError(err.to_string()))?
        .ok_or(ApiError::Forbidden)?;

    match verify_password(&user_data.password, &user.password).await {
        true => Ok(Json(user)),
        false => Err(ApiError::NotFound("User not found".to_owned())),
    }
}

async fn post_user(
    State(state): State<AppState>,
    Json(json): Json<CreateUser>,
) -> ApiResult<StatusCode> {
    user_crud::insert(json, &state.db)
        .await
        .map_err(|err| ApiError::InternalServerError(err.to_string()))?;

    Ok(StatusCode::CREATED)
}

async fn put_user(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    Json(update_data): Json<UpdateUser>,
) -> ApiResult<StatusCode> {
    user_crud::update(id, update_data, &state.db)
        .await
        .map_err(|err| ApiError::InternalServerError(err.to_string()))?;

    Ok(StatusCode::CREATED)
}

async fn patch_user(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    Json(update_data): Json<UpdateUserPartial>,
) -> ApiResult<StatusCode> {
    user_crud::update_partial(id, update_data, &state.db)
        .await
        .map_err(|err| ApiError::InternalServerError(err.to_string()))?;

    Ok(StatusCode::CREATED)
}

async fn delete_user(Path(id): Path<Uuid>, State(state): State<AppState>) -> ApiResult<StatusCode> {
    user_crud::delete(id, &state.db)
        .await
        .map_err(|err| ApiError::InternalServerError(err.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
