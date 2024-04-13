mod auth;
mod endpoints;
mod models;
mod requests;
mod user_query;
mod middleware;

use axum::{routing, Router, middleware::from_fn};
use sqlx::{postgres::PgPoolOptions, PgPool, Pool, Postgres};
use tokio::net::TcpListener;
use std::{sync::Arc, time::Duration};
use endpoints::{
    hello_word, handler_404,
    user::{new_user, patch_user, search_user_by_email},
};
use auth::authorize;
use middleware::jwt::validate_jwt;

pub type AppState = Arc<AppData>;

pub struct AppData {
    pub db: PgPool,
}

async fn db_pool() -> Pool<Postgres> {
    let db_url = std::env::var("DATABASE_URL").expect("Error database connection error");
    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_url)
        .await
        .expect("can`t connection database")
}

async fn init_router() -> Router {
    let pool = db_pool().await;
    let state: AppState = Arc::new(AppData { db: pool });

    Router::new()
        .route("/", routing::get(hello_word))
        .route("/users", routing::post(new_user))
        .route("/users/:id", routing::patch(patch_user))
        .route("/users/search/email", routing::post(search_user_by_email))
        .route("/users/search/username", routing::post(search_user_by_email))
        .with_state(state)
        .route_layer(from_fn(validate_jwt))
        .route("/authorize", routing::post(authorize))
        .fallback(handler_404)
}

async fn init_tcp_listener() -> TcpListener {
    let host = std::env::var("HOST").expect("Host don`t set");
    let port = std::env::var("PORT").expect("Port don`t set");
    let addr = format!("{}:{}", host, port);

    TcpListener::bind(addr).await.expect("The address is busy")
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let router = init_router().await;
    let listener = init_tcp_listener().await;
    axum::serve(listener, router).await.unwrap();
}
