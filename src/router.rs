use crate::file_serve::file_serve;
use axum::{routing::get, Router};

pub async fn routes() -> Router {
    Router::new()
        .route("/", get(root))
        .route_service("/file", file_serve())
}

pub async fn root() -> Result<String, String> {
    Ok("Home".to_string())
}
