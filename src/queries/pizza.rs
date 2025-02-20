use super::PgResult;
use crate::models::{
    pizza::{CreatePizza, GetPizzas, Pizza, UpdatePizza, UpdatePizzaPartial},
    Pagination,
};
use sqlx::{postgres::PgQueryResult, query, query_as, PgPool};

pub async fn select_pizzas(pagination: Pagination, pool: &PgPool) -> PgResult<GetPizzas> {
    let result = GetPizzas {
        pizzas: query_as!(
            Pizza,
            "SELECT * FROM Pizza ORDER BY id LIMIT $1 OFFSET $2;",
            pagination.limit,
            pagination.offset
        )
        .fetch_all(pool)
        .await?,
    };

    Ok(result)
}

pub async fn select_pizza(id: i32, pool: &PgPool) -> PgResult<Option<Pizza>> {
    query_as!(Pizza, "SELECT * FROM Pizza WHERE id = $1;", id)
        .fetch_optional(pool)
        .await
}

pub async fn insert_pizza(new_pizza: CreatePizza, pool: &PgPool) -> PgResult<PgQueryResult> {
    query!(
        "INSERT INTO Pizza (title, ingredients, price) VALUES ($1, $2, $3);",
        new_pizza.title,
        &new_pizza.ingredients,
        new_pizza.price,
    )
    .execute(pool)
    .await
}

pub async fn put_update_pizza(
    id: i32,
    update_data: UpdatePizza,
    pool: &PgPool,
) -> PgResult<PgQueryResult> {
    query!(
        "UPDATE Pizza
        SET title = $1,
            ingredients = $2,
            price = $3
        WHERE id = $4;",
        update_data.title,
        &update_data.ingredients,
        update_data.price,
        id
    )
    .execute(pool)
    .await
}

pub async fn patch_update_pizza(
    id: i32,
    update_data: UpdatePizzaPartial,
    pool: &PgPool,
) -> PgResult<PgQueryResult> {
    query!(
        "UPDATE Pizza
        SET title = COALESCE($1,title),
            ingredients = COALESCE($2, ingredients),
            price = COALESCE($3, price)
        WHERE id = $4;",
        update_data.title,
        update_data.ingredients.as_deref(),
        update_data.price,
        id
    )
    .execute(pool)
    .await
}

pub async fn delete_pizza(id: i32, pool: &PgPool) -> PgResult<PgQueryResult> {
    query!("DELETE FROM Pizza WHERE id = $1;", id)
        .execute(pool)
        .await
}
