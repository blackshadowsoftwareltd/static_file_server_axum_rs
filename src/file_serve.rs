use std::path::PathBuf;
use tower_http::services::{ServeDir, ServeFile};

pub fn serve_file() -> ServeFile {
    let path = PathBuf::from("src/main.rs");
    ServeFile::new(path)
}

pub fn serve_dir() -> ServeDir {
    ServeDir::new("src")
}
