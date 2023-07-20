mod dataset;
mod game;
mod test;

use crate::repository::LaerningToolRepository;

use axum::routing::IntoMakeService;
use axum::{routing::get, Router};
use std::sync::Arc;

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
    return ApiInstance {
        state: Arc::new(ApiState { repository }),
    };
}

impl ApiInstance {
    async fn hello_world() -> &'static str {
        "Hello, World!"
    }

    pub async fn build_router(self) -> Router {
        let router = Router::new().route("/", get(ApiInstance::hello_world));
        let router = game::add_game_route(self.state.clone(), router).await;
        let router = dataset::add_dataset_route(self.state.clone(), router).await;
        router
    }

    pub async fn make_server(self) -> IntoMakeService<Router> {
        self.build_router().await.into_make_service()
    }
}
