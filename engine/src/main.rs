#![recursion_limit = "256"]

mod api;
mod repository;
mod xml;

use crate::xml::error::Error;
use crate::xml::LearningModule;

use api::cli::ToolArgs;
use axum::Server;
use clap::Parser;
use std::net::SocketAddr;
use std::str::FromStr;
use surrealdb::engine::local::{Db, Mem};

use crate::repository::dataset::Dataset;
use crate::repository::{LaerningToolRepository, Repository};
use surrealdb::Surreal;

async fn load_data(directory: &str) -> Vec<LearningModule> {
    xml::list_modules(directory).unwrap()
}

async fn start_db(addr: Option<String>) -> Surreal<Db> {
    let db: Surreal<Db> = if let Some(_address) = addr {
        Surreal::new::<Mem>(()).await.unwrap()
    } else {
        Surreal::new::<Mem>(()).await.unwrap()
    };

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

async fn start_server(
    repository: LaerningToolRepository,
    socket_addr: Option<String>,
) -> Result<(), Error> {
    // Create a new Axum router
    let api_state = api::new(repository);
    let app = api_state.make_server().await;

    // Define the address on which the server will listen
    let addr = if let Some(address) = socket_addr {
        SocketAddr::from_str(&address)
            .map_or(SocketAddr::from(([127, 0, 0, 1], 3000)), |address| address)
    } else {
        SocketAddr::from(([127, 0, 0, 1], 3000))
    };

    // Start the server
    println!("Server running on http://{}", addr);
    println!("Swagger UI available at: http://{}/swagger-ui/#/", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let args = ToolArgs::parse();

    let data = if let Some(directory) = args.directory {
        load_data(&directory).await
    } else {
        Vec::<LearningModule>::new()
    };

    let db = start_db(args.db_location).await;

    let repository = LaerningToolRepository::new(db);
    repository
        .create_batch_datasets(
            data.into_iter()
                .map(|module| Dataset::from(module))
                .collect::<Vec<Dataset>>(),
        )
        .await
        .unwrap();

    start_server(repository, args.bind)
        .await
        .map_err(|e| Error::ServerError(e.to_string()))
        .unwrap();
}
