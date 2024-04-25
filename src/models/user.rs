use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub avatar_url: Option<String>
}

#[derive(Debug, Deserialize, FromRow)]
pub struct CheckUserByEmail {
    pub email: String,
    pub password: String
}

#[derive(Debug, Deserialize, FromRow)]
pub struct CheckUserByUsername {
    pub username: String,
    pub password: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password: String
}

#[derive(Debug, Deserialize)]
pub struct PutUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub address: String,
    pub phone: String,
    pub avatar_url: String
}

#[derive(Debug, Deserialize)]
pub struct PatchUser {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub avatar_url: Option<String>
}