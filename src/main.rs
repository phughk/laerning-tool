use axum::{routing::get, Router};
use hyper::Server;
use std::net::SocketAddr;

async fn hello_world() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() {
    // Create a new Axum router
    let app = Router::new().route("/", get(hello_world));

    // Define the address on which the server will listen
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // Start the server
    println!("Server running on http://{}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
