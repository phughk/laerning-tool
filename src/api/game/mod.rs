mod game;

use crate::api::game::game::Game;
use axum::{routing::get, Json, Router};

async fn game_new() -> Json<Game> {
    Json(Game {
        name: "new game name".to_string(),
        dataset: "new game dataset".to_string(),
    })
}

async fn game_list() -> Json<Game> {
    Json(Game {
        name: "list game name".to_string(),
        dataset: "list game dataset".to_string(),
    })
}

pub async fn add_game_route(router: Router) -> Router {
    router
        .route("/game/new", get(game_new))
        .route("/game/list", get(game_list))
}
