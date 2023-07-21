pub(crate) mod game;

use crate::api::game::game::{Game, GameListing, GameStats, GameStatus};
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
        current_question: None,
        stats: GameStats {
            current_question: 1,
            total_questions: 2,
            current_try: 3,
            max_tries: 4,
            duration: 5,
            average_question_duration: 6.0,
        },
    })
}

#[utoipa::path(
    get,
    path = "/game/list",
    responses(
        (status = 200, description = "Games successfully listed", body = [GameListing]),
    )
)]
pub async fn game_list(state: Extension<Arc<ApiState>>) -> Json<Vec<GameListing>> {
    let state = state.0.clone();
    let games = state.clone().repository.list_games().await.unwrap();
    let _size = games.len();
    println!("The list is {:?}", games);

    Json(
        games
            .into_iter()
            .map(|game| GameListing {
                name: game.name,
                dataset: format!("{:?}", game.id),
                started: "".to_string(),
                status: GameStatus::Pending,
            })
            .collect(),
    )
}
