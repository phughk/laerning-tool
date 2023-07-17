use serde::{Deserialize, Serialize};

use crate::repository::LaerningToolRepository;
use std::collections::HashMap;

use surrealdb::sql::{Strand, Thing, Value};

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub id: Option<Thing>,
    pub name: String,
}

#[derive(Debug)]
pub enum GameError {
    CreateGameFailed,
}

impl LaerningToolRepository {
    pub async fn create_game(&self, game: Game) -> Result<Game, GameError> {
        let mut map: HashMap<String, Value> = HashMap::new();
        map.insert(
            "id".to_string(),
            game.id
                .map(|thing| Value::Thing(thing))
                .unwrap_or(Value::None),
        );
        map.insert("name".to_string(), Value::Strand(Strand::from(game.name)));
        let data: Option<Game> = self
            .db
            .query(r#"INSERT INTO game {"id": $id, "name": $name}"#)
            .bind(map)
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
}
