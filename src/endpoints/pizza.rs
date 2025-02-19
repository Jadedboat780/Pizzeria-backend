use axum::{extract::{Path, State}, Json, Router, routing, http::StatusCode};
use axum::extract::Query;
use crate::api_response::{ApiError, ApiResult};
use crate::AppState;
use crate::models::Pagination;
use crate::models::pizza::{Pizza, CreatePizza, GetPizzas, UpdatePizza, UpdatePizzaPartial};
use crate::queries::pizza::{self, select_pizzas, select_pizza, insert_pizza, patch_update_pizza, put_update_pizza};

pub async fn router_pizza(state: AppState) -> Router {
    Router::new()
        .route("/", routing::get(get_pizzas).post(post_pizza))
        .route("/:id", routing::get(get_pizza).put(put_pizza).patch(patch_pizza).delete(delete_pizza))
        .with_state(state)
}

async fn get_pizzas(
    pagination: Option<Query<Pagination>>,
    State(state): State<AppState>,
) -> ApiResult<Json<GetPizzas>> {
    let Query(pagination) = pagination.unwrap_or_default();
    println!("{:?}", pagination);
    let result = select_pizzas(pagination, &state.db)
        .await
        .map_err(|err| ApiError::InternalServerError(err.to_string()))?;

    Ok(Json(result))

}

async fn get_pizza(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> ApiResult<Json<Pizza>> {
    let result = select_pizza(id, &state.db)
        .await
        .map_err(|err| ApiError::InternalServerError(err.to_string()))?
        .ok_or(ApiError::Forbidden)?;

    Ok(Json(result))
}

async fn post_pizza(
    State(state): State<AppState>,
    Json(json): Json<CreatePizza>,
) -> ApiResult<StatusCode> {
    insert_pizza(json, &state.db)
        .await
        .map_err(|err| ApiError::InternalServerError(err.to_string()))?;

    Ok(StatusCode::CREATED)
}

async fn put_pizza(
    Path(id): Path<i32>,
    State(state): State<AppState>,
    Json(update_data): Json<UpdatePizza>,
) -> ApiResult<StatusCode> {
    put_update_pizza(id, update_data, &state.db)
        .await
        .map_err(|err| ApiError::InternalServerError(err.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}

async fn patch_pizza(
    Path(id): Path<i32>,
    State(state): State<AppState>,
    Json(update_data): Json<UpdatePizzaPartial>,
) -> ApiResult<StatusCode> {
    patch_update_pizza(id, update_data, &state.db)
        .await
        .map_err(|err| ApiError::InternalServerError(err.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}

async fn delete_pizza(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> ApiResult<StatusCode> {
    pizza::delete_pizza(id, &state.db)
        .await
        .map_err(|err| ApiError::InternalServerError(err.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}