mod game;

use crate::api::game::game::Game;
use crate::api::ApiState;

use axum::{routing::get, Extension, Json, Router};

use std::sync::Arc;
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_swagger_ui::SwaggerUi;

#[utoipa::path(
    get,
    path = "/game/new",
    responses(
        (status = 201, description = "Todo item created successfully", body = Todo),
        (status = 409, description = "Todo already exists", body = TodoError)
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
async fn game_list(state: Extension<Arc<ApiState>>) -> Json<Game> {
    let state = state.0.clone();
    let games = state.clone().repository.list_games().await.unwrap();
    let size = games.len();
    println!("The list is {:?}", games);

    Json(Game {
        name: format!("list game name {size}").to_string(),
        dataset: format!("list game dataset {games:?}").to_string(),
    })
}

#[derive(OpenApi)]
#[openapi(
paths(
crate::api::game::game_new,
crate::api::game::game_list,
),
components(
schemas(Game)
),
tags(
(name = "game", description = "Manage game sessions")
)
)]
pub struct ApiDoc;

pub async fn add_game_route(state: Arc<ApiState>, router: Router) -> Router {
    router
        .route("/game/new", get(game_new).layer(Extension(state.clone())))
        .route("/game/list", get(game_list).layer(Extension(state.clone())))
}
