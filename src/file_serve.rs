use std::path::PathBuf;

use tower_http::services::ServeFile;

pub fn file_serve() -> ServeFile {
    let path = PathBuf::from("src/main.rs");
    println!("Serving files from: {:?}", path);
    ServeFile::new(path)
}
