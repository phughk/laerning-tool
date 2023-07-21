use crate::api::ApiState;
use axum::{routing::get, Extension, Json, Router};
use serde::Serialize;
use std::sync::Arc;
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};
use utoipa::{IntoParams, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

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
