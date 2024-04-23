use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    username: String,
    email: String,
    password: String,
    address: Option<String>,
    phone: Option<String>,
    avatar_url: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password: String
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct GetUserByEmail {
    pub email: String,
    pub password: String
}

#[derive(Debug, Deserialize, FromRow)]
pub struct GetUserByUsername {
    pub username: String,
    pub password: String
}

#[derive(Debug, Deserialize)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub avatar_url: Option<String>
}