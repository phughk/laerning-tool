use crate::api::ApiState;

use axum::extract::State;
use axum::Json;
use serde::Serialize;
use std::sync::Arc;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct Dataset {
    pub id: String,
    pub name: String,
}

#[utoipa::path(
    get,
    path = "/dataset/list",
    responses(
        (status = 200, description = "Successfully listed the datasets", body = Vec<Dataset>),
    )
)]
pub async fn dataset_list(State(state): State<Arc<ApiState>>) -> Json<Vec<Dataset>> {
    Json(
        state
            .repository
            .list_datasets()
            .await
            .into_iter()
            .map(|dataset| Dataset {
                id: dataset.id.unwrap().id.to_string(),
                name: dataset.metadata.name.to_string(),
            })
            .collect(),
    )
}
