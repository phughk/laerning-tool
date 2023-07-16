mod api;
mod xml;

use crate::xml::LearningModule;

use hyper::Server;
use std::net::SocketAddr;
use surrealdb::engine::local::{Db, Mem};

use surrealdb::Surreal;

async fn load_data() -> Vec<LearningModule> {
    let modules = xml::list_modules("data").unwrap();
    modules
}

async fn start_db() -> Surreal<Db> {
    let db: Surreal<Db> = Surreal::new::<Mem>(()).await.unwrap();

    // Auth not supported in memory
    /*
    db.signin(Root {
        username: "root",
        password: "root",
    }).await.unwrap();
    */
    db.use_ns("learning-tool-ns")
        .use_db("main-app")
        .await
        .unwrap();

    db
}

async fn start_server(db: Surreal<Db>, data: Vec<LearningModule>) {
    // Create a new Axum router
    let api_state = api::new(db, data);
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
    let db = start_db().await;
    start_server(db, data).await;
}
