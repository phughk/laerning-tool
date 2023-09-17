#![recursion_limit = "256"]
mod api;
mod repository;
mod xml;

use crate::xml::LearningModule;

use hyper::Server;
use std::net::SocketAddr;
use surrealdb::engine::local::{Db, Mem};

use crate::repository::dataset::Dataset;
use crate::repository::{LaerningToolRepository, Repository};
use surrealdb::Surreal;

async fn load_data() -> Vec<LearningModule> {
    xml::list_modules("engine/data").unwrap()
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

async fn start_server(repository: LaerningToolRepository) {
    // Create a new Axum router
    let api_state = api::new(repository);
    let app = api_state.make_server().await;

    // Define the address on which the server will listen
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // Start the server
    println!("Server running on http://{}", addr);
    println!("Swagger UI available at: http://{}/swagger-ui/#/", addr);
    Server::bind(&addr).serve(app).await.unwrap();
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let data = load_data().await;
    let db = start_db().await;
    let repository = LaerningToolRepository::new(db);
    repository
        .create_batch_datasets(
            data.into_iter()
                .map(|module| Dataset::from(module))
                .collect::<Vec<Dataset>>(),
        )
        .await
        .unwrap();
    start_server(repository).await;
}
