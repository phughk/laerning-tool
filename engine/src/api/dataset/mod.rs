use crate::api::ApiState;
use crate::repository::dataset::Dataset;
use crate::repository::Repository;

use axum::extract::State;
use axum::Json;
use serde::Serialize;
use std::sync::Arc;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct DatasetJson {
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
pub async fn dataset_list(State(state): State<Arc<ApiState>>) -> Json<Vec<DatasetJson>> {
    Json(
        state
            .repository
            .list_nature()
            .await
            .unwrap()
            .into_iter()
            .map(|dataset: Dataset| DatasetJson {
                id: dataset.id.unwrap().id.to_string(),
                name: dataset.metadata.name.to_string(),
            })
            .collect(),
    )
}
