use std::sync::Arc;

use axum::extract::State;

use axum::Json;

use surrealdb::sql::Thing;

use crate::api::game::error::{
    GameListingError, GameListingErrorResponse, NewGameError, NewGameErrorResponse,
};
use crate::api::game::game::{Game, GameListing, GameStats, GameStatus, NewGameRequest};
use crate::api::ApiState;
use crate::repository::dataset::DatasetError;
use axum::extract::Path;
use axum::http::Response;
use axum::response::ErrorResponse;
use log::trace;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub mod error;
pub mod game;

#[utoipa::path(
    post,
    path = "/game/new",
    request_body=NewGameRequest,
    responses(
        (status = 201, description = "Game created successfully", body = Game),
        (status = 400, description = "Bad Request", body = NewGameErrorResponse),
        (status = 404, description = "Dataset Not Found", body = NewGameErrorResponse),
    )
)]
pub async fn game_new(
    State(state): State<Arc<ApiState>>,
    Json(request): Json<NewGameRequest>,
) -> Result<Json<Game>, NewGameErrorResponse> {
    if request.dataset.is_empty() {
        return Err(NewGameError::UnspecifiedDataset.into());
    }
    let state = state.clone();

    let dataset = state
        .repository
        .get_dataset(request.dataset.as_str())
        .await
        .or_else(|e| {
            Err(Into::<NewGameErrorResponse>::into(match e {
                DatasetError::UnexpectedDatasetError { cause } => {
                    NewGameError::InternalError { cause }
                }
                // TODO this is a bit silly, it won't happen
                DatasetError::CreateDatasetFailed => NewGameError::InternalError {
                    cause: "Failed to create dataset during new game creation".to_string(),
                },
            }))
        })?;

    if let None = dataset {
        return Err(NewGameErrorResponse {
            code: 500,
            cause: NewGameError::UnrecognisedDataset {
                dataset: request.dataset,
            },
        }
        .into());
    }

    let created_game = state
        .repository
        .create_game(crate::repository::game::Game {
            id: request
                .name
                .map(|id_str| Thing::from(("game".to_string(), id_str))),
            dataset: Thing::from(("game", request.dataset.as_str())),
        })
        .await
        .unwrap();

    Ok(Json(Game {
        name: created_game.id.map(|thing| thing.id.to_string()).ok_or(
            NewGameError::InternalError {
                cause: "Missing ID after creation".to_string(),
            },
        )?,
        dataset: created_game.dataset.id.to_string(),
        current_question: None,
        stats: GameStats {
            current_question: 1,
            total_questions: 2,
            current_try: 3,
            max_tries: 4,
            duration: 5,
            average_question_duration: 6.0,
        },
    }))
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GameAnswerRequest {}

#[utoipa::path(
    post,
    path = "/game/{id}",
    request_body=GameAnswerRequest,
    params(
        ("id" = String, Path, description = "Game ID")
    ),
    responses(
        (status = 201, description = "Game responded to successfully", body = Game),
    )
)]
pub async fn game_answer(
    Path(id): Path<String>,
    State(_state): State<Arc<ApiState>>,
    Json(request): Json<GameAnswerRequest>,
) -> Result<Json<Game>, ErrorResponse> {
    trace!("{:?}  {:?}", id, request);
    Ok(Json(Game {
        name: "".to_string(),
        dataset: "".to_string(),
        current_question: None,
        stats: GameStats {
            current_question: 1,
            total_questions: 2,
            current_try: 3,
            max_tries: 4,
            duration: 5,
            average_question_duration: 6.0,
        },
    }))
}

#[utoipa::path(
    get,
    path = "/game/list",
    responses(
        (status = 200, description = "Games successfully listed", body = [GameListing]),
        (status = 500, description = "Unexpected internal error", body = GameListingErrorResponse)
    )
)]
pub async fn game_list(
    State(state): State<Arc<ApiState>>,
) -> Result<Json<Vec<GameListing>>, GameListingErrorResponse> {
    let state = state.clone();
    let games = state.clone().repository.list_games().await.or_else(|e| {
        Err(Into::<GameListingErrorResponse>::into(match e {
            _ => GameListingError::InternalError {
                cause: "Unexpected internal error".to_string(),
            },
        }))
    })?;
    let _size = games.len();
    println!("The list is {:?}", games);

    Ok(Json(
        games
            .into_iter()
            .map(|game| GameListing {
                name: game
                    .id
                    .map(|thing| thing.id.to_string())
                    .unwrap_or("<no name provided>".to_string()),
                dataset: format!("{:?}", game.dataset),
                started: "".to_string(),
                status: GameStatus::Pending,
            })
            .collect(),
    ))
}
