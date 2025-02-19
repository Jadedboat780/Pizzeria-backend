pub mod user;
pub mod pizza;

use axum::body::Body;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use axum::response::Response;
use serde_json::json;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use crate::api_response::ApiError;

pub async fn ping() -> Json<serde_json::Value> {
    Json(json!({"ping": "pong!"}))
}

pub async fn handler_404() -> ApiError {
    ApiError::NotFound("Page not found".to_owned())
}

pub async fn get_file(Path(filename): Path<String>) -> Response<Body> {
    let mut file = File::open(format!("images/{filename}")).await.expect("Failed to open file");
    let mut file_data = Vec::new();
    file.read_to_end(&mut file_data).await.unwrap();

    Response::builder()
        .status(StatusCode::OK)
        // .header("Content-Disposition", format!("attachment; filename=\"{filename}\""))
        .header("Content-Type", "image/png")
        .body(Body::from(file_data))
        .unwrap()
}

// pub async fn upload_image(mut multipart: Multipart) {
//     todo!()
// }