pub mod addr;
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
