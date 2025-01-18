use super::PgResult;
use crate::models::user::{
    User, CheckUserByEmail, CheckUserByUsername, CreateUser, UpdateUser, UpdateUserPartial,
};
use sqlx::{postgres::PgQueryResult, PgPool, query, query_as};
use utils::encryption::hash_password;
use uuid::Uuid;

pub async fn select_user_by_id(id: Uuid, pool: &PgPool) -> PgResult<Option<User>> {
    query_as!(User, "SELECT * FROM Users WHERE id = $1;", id)
        .fetch_optional(pool)
        .await
}

pub async fn select_user_by_email(
    user_data: &CheckUserByEmail,
    pool: &PgPool,
) -> PgResult<Option<User>> {
    query_as!(
        User,
        "SELECT * FROM Users WHERE email = $1;",
        user_data.email
    )
    .fetch_optional(pool)
    .await
}

pub async fn select_user_by_username(
    user_data: &CheckUserByUsername,
    pool: &PgPool,
) -> PgResult<Option<User>> {
    query_as!(
        User,
        "SELECT * FROM Users WHERE username = $1;",
        user_data.username
    )
    .fetch_optional(pool)
    .await
}

pub async fn insert_user(new_user: CreateUser, pool: &PgPool) -> PgResult<PgQueryResult> {
    let hash = hash_password(new_user.password.as_str()).await;
    query!(
        "INSERT INTO Users (username, email, password) VALUES ($1, $2, $3);",
        new_user.username,
        new_user.email,
        hash
    )
    .execute(pool)
    .await
}

pub async fn put_update_user(
    id: Uuid,
    update_data: UpdateUser,
    pool: &PgPool,
) -> PgResult<PgQueryResult> {
    query!(
        "UPDATE Users
        SET username = $1,
            email = $2,
            password = $3,
            address = $4,
            phone = $5
        WHERE id = $6;",
        update_data.username,
        update_data.email,
        update_data.password,
        update_data.address,
        update_data.phone,
        id
    )
    .execute(pool)
    .await
}

pub async fn patch_update_user(
    id: Uuid,
    update_data: UpdateUserPartial,
    pool: &PgPool,
) -> PgResult<PgQueryResult> {
    query!(
        "UPDATE Users
        SET username = COALESCE($1, username),
            email = COALESCE($2, email),
            password = COALESCE($3, password),
            address = COALESCE($4, address),
            phone = COALESCE($5, phone)
        WHERE id = $6;",
        update_data.username,
        update_data.email,
        update_data.password,
        update_data.address,
        update_data.phone,
        id
    )
    .execute(pool)
    .await
}

pub async fn delete_user(id: Uuid, pool: &PgPool) -> PgResult<PgQueryResult> {
    query!("DELETE FROM Users WHERE id = $1;", id)
        .execute(pool)
        .await
}
