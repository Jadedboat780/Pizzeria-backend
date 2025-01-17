use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub address: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CheckUserByEmail {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct CheckUserByUsername {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub address: String,
    pub phone: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserPartial {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
}
