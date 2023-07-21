use serde::{Deserialize, Serialize};

use utoipa::{ToSchema};

#[derive(Deserialize, Serialize, ToSchema)]
pub struct Game {
    pub name: String,
    pub dataset: String,
}
