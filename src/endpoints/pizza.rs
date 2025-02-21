use crate::crud::pizza as pizza_crud;
use crate::models::pizza::{CreatePizza, GetPizzas, Pizza, UpdatePizza, UpdatePizzaPartial};
use crate::models::Pagination;
use crate::AppState;
use api_response::{ApiError, ApiResult};
use axum::extract::Query;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing, Json, Router,
};

pub async fn router(state: AppState) -> Router {
    Router::new()
        .route("/", routing::get(get_pizzas).post(post_pizza))
        .route(
            "/{id}",
            routing::get(get_pizza)
                .put(put_pizza)
                .patch(patch_pizza)
                .delete(delete_pizza),
        )
        .with_state(state)
}

async fn get_pizzas(
    pagination: Query<Pagination>,
    State(state): State<AppState>,
) -> ApiResult<Json<GetPizzas>> {
    let pagination = pagination.0;
    let result = pizza_crud::select_many(pagination, &state.db)
        .await
        .map_err(|err| ApiError::InternalServerError(err.to_string()))?;

    Ok(Json(result))
}

async fn get_pizza(Path(id): Path<i32>, State(state): State<AppState>) -> ApiResult<Json<Pizza>> {
    let result = pizza_crud::select(id, &state.db)
        .await
        .map_err(|err| ApiError::InternalServerError(err.to_string()))?
        .ok_or(ApiError::Forbidden)?;

    Ok(Json(result))
}

async fn post_pizza(
    State(state): State<AppState>,
    Json(json): Json<CreatePizza>,
) -> ApiResult<StatusCode> {
    pizza_crud::insert(json, &state.db)
        .await
        .map_err(|err| ApiError::InternalServerError(err.to_string()))?;

    Ok(StatusCode::CREATED)
}

async fn put_pizza(
    Path(id): Path<i32>,
    State(state): State<AppState>,
    Json(update_data): Json<UpdatePizza>,
) -> ApiResult<StatusCode> {
    pizza_crud::update(id, update_data, &state.db)
        .await
        .map_err(|err| ApiError::InternalServerError(err.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}

async fn patch_pizza(
    Path(id): Path<i32>,
    State(state): State<AppState>,
    Json(update_data): Json<UpdatePizzaPartial>,
) -> ApiResult<StatusCode> {
    pizza_crud::update_partial(id, update_data, &state.db)
        .await
        .map_err(|err| ApiError::InternalServerError(err.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}

async fn delete_pizza(Path(id): Path<i32>, State(state): State<AppState>) -> ApiResult<StatusCode> {
    pizza_crud::delete(id, &state.db)
        .await
        .map_err(|err| ApiError::InternalServerError(err.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
