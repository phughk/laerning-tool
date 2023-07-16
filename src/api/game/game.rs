use serde::Serialize;
use surrealdb::sql::Thing;

#[derive(Serialize)]
pub struct Game {
    pub name: String,
    pub dataset: String,
}
