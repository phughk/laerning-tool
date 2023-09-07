use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::repository::{LaerningToolRepository, Repository};

use surrealdb::sql::Thing;

use super::RepositoryError;

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

#[async_trait]
impl Repository<Game, GameError> for LaerningToolRepository {
    async fn create_dataset(&self, game: Game) -> Result<Game, RepositoryError<GameError>> {
        let data: Option<Game> = self
            .db
            .query(r#"INSERT INTO game $game"#)
            .bind(("game", &game))
            .await
            .map_err(|_| RepositoryError::CreationFailed(GameError::CreateGameFailed))
            .unwrap()
            .take(0)
            .unwrap();

        Ok(data.unwrap())
    }

    async fn create_batch_datasets(&self, datasets: Vec<Game>) -> Result<(), RepositoryError<GameError>> {
        for dataset in datasets {
            self.create_dataset(dataset).await.unwrap();
        }
        Ok(())
    }

    async fn create_list(&self) -> Result<Vec<Game>, RepositoryError<GameError>> {
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
