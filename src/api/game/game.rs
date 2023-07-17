use serde::Serialize;


#[derive(Serialize)]
pub struct Game {
    pub name: String,
    pub dataset: String,
}
