use crate::file_serve::{serve_dir, serve_file};
use axum::{routing::get, Router};

pub async fn routes() -> Router {
    Router::new()
        .route("/", get(root))
        .route_service("/file", serve_file())
        .nest_service("/files", serve_dir())
}

pub async fn root() -> Result<String, String> {
    Ok("Home".to_string())
}
