pub(crate) mod dataset;
pub(crate) mod game;

use async_trait::async_trait;
use serde::Deserialize;
use surrealdb::engine::local::Db;
use surrealdb::Surreal;

pub struct LaerningToolRepository {
    db: Surreal<Db>,
}

impl LaerningToolRepository {
    pub fn new(db: Surreal<Db>) -> Self {
        Self { db }
    }
}

#[derive(Deserialize, Debug)]
pub enum RepositoryError<U> {
    CreationFailed(U),
}

#[async_trait]
pub trait Repository<T, U> {
    async fn create(&self, nature: T) -> Result<T, RepositoryError<U>>;

    async fn create_batch(&self, nature: Vec<T>) -> Result<(), RepositoryError<U>>;

    async fn list_nature(&self) -> Result<Vec<T>, RepositoryError<U>>;
}
