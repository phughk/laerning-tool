use serde::Serialize;

use utoipa::{IntoParams, ToSchema};

#[derive(Serialize, ToSchema)]
pub struct Game {
    pub name: String,
    pub dataset: String,
}
