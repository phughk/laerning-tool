mod game;
mod test;

use crate::xml::LearningModule;
use axum::routing::IntoMakeService;
use axum::{routing::get, Router};
use std::sync::Arc;
use surrealdb::engine::local::{Db};
use surrealdb::Surreal;

/// ApiState is all the information required to use the application
pub struct ApiState {
    pub(crate) db: Surreal<Db>,
    pub(crate) modules: Vec<LearningModule>,
}

/// ApiInstance is a way of storing all the state of the application
/// without using static state to share data between requests
pub struct ApiInstance {
    state: Arc<ApiState>,
}

/// Create new ApiInstance that tracks and owns state
pub fn new(db: Surreal<Db>, modules: Vec<LearningModule>) -> ApiInstance {
    return ApiInstance {
        state: Arc::new(ApiState { db, modules }),
    };
}

impl ApiInstance {
    async fn hello_world() -> &'static str {
        "Hello, World!"
    }

    pub async fn build_router(self) -> Router {
        let router = Router::new().route("/", get(ApiInstance::hello_world));
        let router = game::add_game_route(self.state.clone(), router).await;
        router
    }

    pub async fn make_server(self) -> IntoMakeService<Router> {
        self.build_router().await.into_make_service()
    }
}
