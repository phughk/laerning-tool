use crate::repository::RepositoryError::CreateGameFailed;
use serde::Deserialize;

use std::collections::HashMap;
use surrealdb::engine::local::Db;
use surrealdb::sql::Thing;
use surrealdb::{Surreal};

pub struct LaerningToolRepository {
    db: Surreal<Db>,
}

pub fn new(db: Surreal<Db>) -> LaerningToolRepository {
    LaerningToolRepository { db }
}

#[derive(Debug, Deserialize)]
pub struct Game {
    pub id: Thing,
    pub name: String,
}

#[derive(Debug)]
pub enum RepositoryError {
    CreateGameFailed,
}

impl LaerningToolRepository {
    pub async fn create_game(&self, game: Game) -> Result<Game, RepositoryError> {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("id".to_string(), game.id.to_string());
        map.insert("name".to_string(), game.name.to_string());
        let data: Option<Game> = self
            .db
            .query(r#"INSERT INTO game {"id": $id, "name": $name}"#)
            .bind(map)
            .await
            .unwrap()
            .take(0)
            .unwrap();
        data.ok_or(CreateGameFailed)
    }

    pub async fn list_games(&self) -> Result<Vec<Game>, RepositoryError> {
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
