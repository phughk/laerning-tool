mod api;
mod xml;

use crate::xml::LearningModule;
use axum::{routing::get, Router};
use hyper::Server;
use std::net::SocketAddr;

async fn load_data() -> Vec<LearningModule> {
    let modules = xml::list_modules("data").unwrap();
    modules
}

async fn start_server(data: Vec<LearningModule>) {
    // Create a new Axum router
    let api_state = api::new(data);
    let app = api_state.make_server().await;

    // Define the address on which the server will listen
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // Start the server
    println!("Server running on http://{}", addr);
    Server::bind(&addr).serve(app).await.unwrap();
}

#[tokio::main]
async fn main() {
    let data = load_data().await;
    start_server(data).await;
}
