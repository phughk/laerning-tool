mod game;

use crate::api::game::game::Game;
use crate::api::ApiState;

use axum::{routing::get, Extension, Json, Router};
use serde::Deserialize;
use std::sync::Arc;
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize)]
struct UserType {
    id: Thing,
    name: String,
}

pub async fn game_new(state: Extension<Arc<ApiState>>) -> Json<Game> {
    let size = state.clone().modules.len();
    let data: Option<UserType> = state
        .clone()
        .db
        .query(r#"INSERT INTO user {"name": "Hugh"}"#)
        .await
        .unwrap()
        .take(0)
        .unwrap();
    println!("The data is {data:?}");

    Json(Game {
        name: format!("new game name {size} {data:?}").to_string(),
        dataset: format!("new game dataset {size}").to_string(),
    })
}

async fn game_list(state: Extension<Arc<ApiState>>) -> Json<Game> {
    let size = state.clone().modules.len();
    Json(Game {
        name: format!("list game name {size}").to_string(),
        dataset: format!("list game dataset {size}").to_string(),
    })
}

pub async fn add_game_route(state: Arc<ApiState>, router: Router) -> Router {
    router
        .route("/game/new", get(game_new).layer(Extension(state.clone())))
        .route("/game/list", get(game_list).layer(Extension(state.clone())))
}
