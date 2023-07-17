use crate::repository::dataset::DatasetError::CreateDatasetFailed;
use crate::repository::LaerningToolRepository;
use axum::handler::Handler;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use surrealdb::sql;
use surrealdb::sql::Value::Thing;
use surrealdb::sql::{Object, Value};

#[derive(Serialize, Deserialize, Debug)]
pub struct Dataset {
    id: Option<sql::Thing>,
}

impl Into<Object> for Dataset {
    fn into(self) -> Object {
        let mut map: BTreeMap<String, Value> = BTreeMap::new();
        map.insert(
            "id".to_string(),
            self.id.map_or(Value::None, |thing| Value::Thing(thing)),
        );
        Object::from(map)
    }
}

#[derive(Deserialize, Debug)]
pub enum DatasetError {
    CreateDatasetFailed,
}

impl LaerningToolRepository {
    pub async fn create_dataset(&self, dataset: Dataset) -> Result<Dataset, DatasetError> {
        let mut bindings: HashMap<String, Value> = HashMap::new();
        bindings.insert("data".to_string(), Value::Object(dataset.into()));
        let created: Option<Dataset> = self
            .db
            .query("INSERT INTO dataset $data")
            .bind(bindings)
            .await
            .unwrap()
            .take(0)
            .unwrap();
        created.ok_or(CreateDatasetFailed)
    }
}
