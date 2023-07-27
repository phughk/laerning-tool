use serde::{Deserialize, Serialize};

use crate::repository::LaerningToolRepository;
use std::collections::HashMap;

use surrealdb::sql::{Strand, Thing, Value};

/// A Game is a session that is constructed from a single dataset.
/// It is currently tied to a single user, but down the line may be tied with several.
#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub id: Option<Thing>,
    pub dataset: Thing,
}

#[derive(Debug)]
pub enum GameError {
    CreateGameFailed,
}

impl LaerningToolRepository {
    pub async fn create_game(&self, game: Game) -> Result<Game, GameError> {
        let data: Option<Game> = self
            .db
            .query(r#"INSERT INTO game $game"#)
            .bind(("game", &game))
            .await
            .unwrap()
            .take(0)
            .unwrap();
        data.ok_or(GameError::CreateGameFailed)
    }

    pub async fn list_games(&self) -> Result<Vec<Game>, GameError> {
        let data: Vec<Game> = self
            .db
            .query(r#"SELECT * FROM game LIMIT 1000"#)
            .await
            .unwrap()
            .take(0)
            .unwrap();
        Ok(data)
    }

    pub async fn post_answer(&self) -> Result<Game, AnswerError> {}
}
