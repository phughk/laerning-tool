use crate::api::ApiState;
use axum::{routing::get, Extension, Json, Router};
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize)]
pub struct Dataset {
    pub name: String,
}

pub async fn dataset_list(state: Extension<Arc<ApiState>>) -> Json<Vec<Dataset>> {
    Json(
        state
            .repository
            .list_datasets()
            .await
            .into_iter()
            .map(|dataset| Dataset {
                name: dataset.metadata.name.to_string(),
            })
            .collect(),
    )
}

pub async fn add_dataset_route(state: Arc<ApiState>, router: Router) -> Router {
    router.route(
        "/dataset/list",
        get(dataset_list).layer(Extension(state.clone())),
    )
}
