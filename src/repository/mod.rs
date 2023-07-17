mod dataset;
pub(crate) mod game;

use surrealdb::engine::local::Db;
use surrealdb::Surreal;

pub struct LaerningToolRepository {
    db: Surreal<Db>,
}

pub fn new(db: Surreal<Db>) -> LaerningToolRepository {
    LaerningToolRepository { db }
}

impl LaerningToolRepository {}
