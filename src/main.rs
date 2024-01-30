pub mod addr;
pub mod error;
pub mod file_serve;
pub mod router;
use crate::addr::ip_address;
use crate::router::routes;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = routes().await;

    let addr = ip_address().await;
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("File Server Listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}

// use std::net::SocketAddr;

// use axum::Router;
// use tower_http::services::{ServeDir, ServeFile};
// use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
// #[tokio::main]
// async fn main() {
//     tracing_subscriber::registry()
//         .with(
//             tracing_subscriber::EnvFilter::try_from_default_env()
//                 .unwrap_or_else(|_| "example_static_file_server=debug,tower_http=debug".into()),
//         )
//         .with(tracing_subscriber::fmt::layer())
//         .init();

//     tokio::join!(
//         serve(using_serve_dir(), 3001),
//         // serve(using_serve_dir_with_assets_fallback(), 3002),
//         // serve(using_serve_dir_only_from_root_via_fallback(), 3003),
//         // serve(using_serve_dir_with_handler_as_service(), 3004),
//         // serve(two_serve_dirs(), 3005),
//         // serve(calling_serve_dir_from_a_handler(), 3006),
//         serve(using_serve_file_from_a_route(), 3307),
//     );
// }
// fn using_serve_dir() -> Router {
//     // serve the file in the "assets" directory under `/assets`
//     Router::new().nest_service("/assets", ServeDir::new("assets"))
// }
// fn using_serve_file_from_a_route() -> Router {
//     Router::new().route_service("/foo", ServeFile::new("assets/Cargo.lock"))
// }
// async fn serve(app: Router, port: u16) {
//     let addr = SocketAddr::from(([127, 0, 0, 1], port));
//     let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
//     tracing::debug!("listening on {}", listener.local_addr().unwrap());
//     println!("File Server Listening on {}/assets", addr);
//     axum::serve(listener, app).await.unwrap();
// }
