use axum::{routing::get, Router};

async fn game_new() -> &'static str {
    "Game, new!"
}

async fn game_list() -> &'static str {
    "Game, list!"
}

pub fn add_game_route(router: Router) -> Router {
    router
        .route("/game/new", get(game_new))
        .route("/game/list", get(game_list))
}
