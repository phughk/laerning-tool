use crate::api::ApiState;
use axum::{Extension, Json};
use serde::Serialize;
use std::sync::Arc;

use utoipa::{ToSchema};


#[derive(Serialize, ToSchema)]
pub struct Dataset {
    pub id: String,
    pub name: String,
}

#[utoipa::path(
    get,
    path = "/dataset/list",
    responses(
        (status = 201, description = "Todo item created successfully", body = Vec<Dataset>),
    )
)]
pub async fn dataset_list(state: Extension<Arc<ApiState>>) -> Json<Vec<Dataset>> {
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
