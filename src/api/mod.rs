mod game;
mod test;

use axum::routing::IntoMakeService;
use axum::{routing::get, Router};

async fn hello_world() -> &'static str {
    "Hello, World!"
}

pub async fn build_router() -> Router {
    let router = Router::new().route("/", get(hello_world));
    let router = game::add_game_route(router).await;
    router
}

pub async fn make_server() -> IntoMakeService<Router> {
    build_router().await.into_make_service()
}
