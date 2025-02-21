mod crud;
mod endpoints;
mod middleware;
mod models;

use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, ORIGIN},
        Method,
    },
    middleware::from_fn,
    routing, Router,
};
use endpoints::{handler_404, ping, pizza, user};
use sqlx::{postgres::PgPoolOptions, PgPool, Pool, Postgres};
use std::{sync::Arc, time::Duration};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
// use auth::authorize;
// use middleware::jwt::validate_jwt;

pub struct AppData {
    pub db: PgPool,
}

pub type AppState = Arc<AppData>;

pub struct App {
    listener: TcpListener,
    router: Router,
}

impl App {
    pub async fn new() -> Self {
        let listener = Self::init_tcp_listener().await;
        let router = Self::init_router().await;

        App { listener, router }
    }

    async fn init_db_pool() -> Pool<Postgres> {
        let db_url = std::env::var("DATABASE_URL").expect("Error database connection error");

        PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(3))
            .connect(&db_url)
            .await
            .expect("can`t connection database")
    }

    async fn init_cors() -> CorsLayer {
        let origins = ["http://localhost:3000".parse().unwrap()];

        CorsLayer::new()
            .allow_methods([
                Method::GET,
                Method::POST,
                Method::PUT,
                Method::PATCH,
                Method::DELETE,
            ])
            .allow_headers([ORIGIN, AUTHORIZATION, ACCEPT, CONTENT_TYPE])
            .allow_origin(origins)
    }

    pub async fn init_router() -> Router {
        let pool = Self::init_db_pool().await;
        let state: AppState = Arc::new(AppData { db: pool });
        let cors = Self::init_cors().await;
        let user_router = user::router(state.clone()).await;
        let pizza_router = pizza::router(state.clone()).await;

        Router::new()
            .route("/ping", routing::get(ping))
            .nest("/user", user_router)
            .nest("/pizza", pizza_router)
            // .route("/image/{name}", routing::get(get_file))
            // .route_layer(from_fn(validate_jwt))
            // .route("/authorize", routing::post(authorize))
            .fallback(handler_404)
            .layer(cors)
    }

    pub async fn init_tcp_listener() -> TcpListener {
        let host = std::env::var("HOST").expect("Host don`t set");
        let port = std::env::var("PORT").expect("Port don`t set");
        let addr = format!("{}:{}", host, port);

        TcpListener::bind(addr).await.expect("the address is busy")
    }

    pub async fn run(self) {
        axum::serve(self.listener, self.router).await.unwrap()
    }
}
