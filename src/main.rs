mod api;
mod xml;

use axum::{routing::get, Router};
use hyper::Server;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Create a new Axum router
    let app = api::make_server().await;

    // Define the address on which the server will listen
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // Start the server
    println!("Server running on http://{}", addr);
    Server::bind(&addr).serve(app).await.unwrap();
}
