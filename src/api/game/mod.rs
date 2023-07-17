mod game;

use crate::api::game::game::Game;
use crate::api::ApiState;

use axum::{routing::get, Extension, Json, Router};
use serde::Deserialize;
use std::sync::Arc;
use surrealdb::sql::{Id, Thing};

pub async fn game_new(state: Extension<Arc<ApiState>>) -> Json<Game> {
    let state = state.0.clone();
    let size = state.modules.len();
    let created_game = state
        .repository
        .create_game(crate::repository::Game {
            id: Thing {
                tb: "".to_string(),
                id: (Id::String("".to_string())),
            },
            name: "".to_string(),
        })
        .await
        .unwrap();
    println!("The data is {:?}", created_game);

    Json(Game {
        name: format!("new game name {size} {created_game:?}").to_string(),
        dataset: format!("new game dataset {size}").to_string(),
    })
}

async fn game_list(state: Extension<Arc<ApiState>>) -> Json<Game> {
    let state = state.0.clone();
    let size = state.modules.len();
    let games = state.clone().repository.list_games().await.unwrap();
    println!("The list is {:?}", games);

    Json(Game {
        name: format!("list game name {size}").to_string(),
        dataset: format!("list game dataset {games:?}").to_string(),
    })
}

pub async fn add_game_route(state: Arc<ApiState>, router: Router) -> Router {
    router
        .route("/game/new", get(game_new).layer(Extension(state.clone())))
        .route("/game/list", get(game_list).layer(Extension(state.clone())))
}
