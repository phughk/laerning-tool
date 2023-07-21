pub(crate) mod game;

use crate::api::game::game::Game;
use crate::api::ApiState;

use axum::{Extension, Json};

use std::sync::Arc;

#[utoipa::path(
    post,
    path = "/game/new",
    responses(
        (status = 201, description = "Game created successfully", body = Game),
    )
)]
pub async fn game_new(state: Extension<Arc<ApiState>>) -> Json<Game> {
    let state = state.0.clone();
    let created_game = state
        .repository
        .create_game(crate::repository::game::Game {
            id: None,
            name: "".to_string(),
        })
        .await
        .unwrap();
    println!("The data is {:?}", created_game);

    Json(Game {
        name: format!("new game name {created_game:?}").to_string(),
        dataset: format!("new game dataset").to_string(),
    })
}

#[utoipa::path(
    get,
    path = "/game/list",
    responses(
        (status = 201, description = "Todo item created successfully", body = Todo),
        (status = 409, description = "Todo already exists", body = TodoError)
    )
)]
pub async fn game_list(state: Extension<Arc<ApiState>>) -> Json<Game> {
    let state = state.0.clone();
    let games = state.clone().repository.list_games().await.unwrap();
    let size = games.len();
    println!("The list is {:?}", games);

    Json(Game {
        name: format!("list game name {size}").to_string(),
        dataset: format!("list game dataset {games:?}").to_string(),
    })
}
