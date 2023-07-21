use serde::{Deserialize, Serialize};

use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, Serialize, ToSchema)]
pub struct Game {
    pub name: String,
    pub dataset: String,
}
