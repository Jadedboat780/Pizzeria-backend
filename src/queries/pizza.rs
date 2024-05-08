use sqlx::{postgres::PgQueryResult, query_file, query_file_as, PgPool};
use crate::queries::PgResult;
use crate::models::Pagination;
use crate::models::pizza::{Pizza, GetPizzas, CreatePizza, PutPizza, PatchPizza};

pub async fn select_pizzas(pagination: Pagination, pool: &PgPool) -> PgResult<GetPizzas> {
    let result = GetPizzas {
        pizzas: query_file_as!(
            Pizza,
            "queries/pizza/select_pizzas.sql",
            pagination.limit,
            pagination.offset
            )
            .fetch_all(pool)
            .await?
    };

    Ok(result)
}

pub async fn select_pizza(id: i32, pool: &PgPool) -> PgResult<Option<Pizza>> {
    query_file_as!(
        Pizza,
        "queries/pizza/select_pizza.sql",
        id
    )
        .fetch_optional(pool)
        .await
}

pub async fn insert_pizza(new_pizza: CreatePizza, pool: &PgPool) -> PgResult<PgQueryResult> {
    query_file!(
        "queries/pizza/insert_pizza.sql",
        new_pizza.title,
        &new_pizza.ingredients,
        new_pizza.price,
        new_pizza.image_url
    )
        .execute(pool)
        .await
}

pub async fn put_update_pizza(id: i32, update_data: PutPizza, pool: &PgPool) -> PgResult<PgQueryResult> {
    query_file!(
        "queries/pizza/put_update_pizza.sql",
        update_data.title,
        &update_data.ingredients,
        update_data.price,
        update_data.image_url,
        id
        )
        .execute(pool)
        .await
}

pub async fn patch_update_pizza(id: i32, update_data: PatchPizza, pool: &PgPool) -> PgResult<PgQueryResult> {
    query_file!(
        "queries/pizza/patch_update_pizza.sql",
        update_data.title,
        update_data.ingredients.as_deref(),
        update_data.price,
        update_data.image_url,
        id
        )
        .execute(pool)
        .await
}

pub async fn delete_pizza(id: i32, pool: &PgPool) -> PgResult<PgQueryResult> {
    query_file!(
        "queries/pizza/delete_pizza.sql",
        id
    )
        .execute(pool)
        .await
}