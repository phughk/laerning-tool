mod game;

use axum::{routing::get, Router};

async fn hello_world() -> &'static str {
    "Hello, World!"
}

pub(crate) fn build_router() -> Router {
    let router = Router::new().route("/", get(hello_world));
    let router = game::add_game_route(router);
    router
}
