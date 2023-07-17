use crate::repository::RepositoryError::{CreateGameFailed, ListGameFailed, NoResults};
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::collections::HashMap;
use surrealdb::engine::local::Db;
use surrealdb::sql::Thing;
use surrealdb::{Error, Surreal};

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
    ListGameFailed { cause: Error },
    NoResults,
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
        // data.and_then(|opt| -> Vec<Game> { opt.unwrap_or(Vec::new()) })
        // data.map_err(|sdb_error| -> RepositoryError { ListGameFailed { cause: sdb_error } })
    }
}
