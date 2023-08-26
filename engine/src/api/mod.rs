mod dataset;
mod game;
mod test;

use crate::repository::LaerningToolRepository;

use crate::api::game::error::{
    GameListingError, GameListingErrorResponse, NewGameError, NewGameErrorResponse,
};
use crate::api::game::game_library::{
    AnswerType, GameJson, GameListing, GameStats, GameStatus, NewGameRequest, QuestionEntry,
};
use crate::api::game::GameAnswerRequest;

use axum::routing::{post, IntoMakeService};
use axum::{routing::get, Router};
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

/// ApiState is all the information required to use the application
pub struct ApiState {
    pub(crate) repository: LaerningToolRepository,
}

/// ApiInstance is a way of storing all the state of the application
/// without using static state to share data between requests
pub struct ApiInstance {
    state: Arc<ApiState>,
}

/// Create new ApiInstance that tracks and owns state
pub fn new(repository: LaerningToolRepository) -> ApiInstance {
    ApiInstance {
        state: Arc::new(ApiState { repository }),
    }
}

// This struct lists all the accessible API paths. For swagger.
// Swagger is not included with path implementation, so we specify and link manually.
#[derive(OpenApi)]
#[openapi(
paths(
    dataset::dataset_list,
    game::game_new,
    game::game_list,
    game::game_answer,
),
components(
    schemas(
        dataset::DatasetJson,
        AnswerType,
        GameJson,
        GameListing,
        GameListingError,
        GameListingErrorResponse,
        GameStats,
        GameStatus,
        NewGameRequest,
        QuestionEntry,
        NewGameErrorResponse,
        NewGameError,
        GameAnswerRequest,
    )
),
tags(
    (name = "this is a tag name", description = "This is the tag description"))
)]
pub struct ApiDoc;

impl ApiInstance {
    pub async fn build_router(self) -> Router {
        Router::new()
            .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
            .route(
                "/dataset/list",
                get(dataset::dataset_list).with_state(self.state.clone()),
            )
            .route(
                "/game/new",
                post(game::game_new).with_state(self.state.clone()),
            )
            .route(
                "/game/list",
                get(game::game_list).with_state(self.state.clone()),
            )
            .route("/game/:id", post(game::game_answer).with_state(self.state))
    }

    pub async fn make_server(self) -> IntoMakeService<Router> {
        self.build_router().await.into_make_service()
    }
}
