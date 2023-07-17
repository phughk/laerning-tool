use crate::repository::dataset::DatasetError::CreateDatasetFailed;
use crate::repository::LaerningToolRepository;
use axum::handler::Handler;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use surrealdb::sql;
use surrealdb::sql::Value::Thing;
use surrealdb::sql::{Number, Object, Strand, Value};

/// A dataset represents a collection of information (and questions) that can be used to generate
/// a Game.
///
/// It includes the question and answer pairs, but also dataset meta data.
/// It is not tied to any single user.
#[derive(Serialize, Deserialize, Debug)]
pub struct Dataset {
    // This is not part of the XML
    id: Option<sql::Thing>,

    // Below is part of the XML
    metadata: Object,
    entries: Vec<DatasetEntry>,
}

/// This describes the metadata of the file
/// It is nested, not independent
#[derive(Serialize, Deserialize, Debug)]
pub struct DatasetMetadata {
    pub name: Strand,
    pub description: Strand,
    pub author: Strand,
    pub updated: Strand,
    pub file_version: Strand,
    pub format_version: Strand,
}

/// This is basically a question in the dataset, but "question" is quite overloaded
#[derive(Serialize, Deserialize, Debug)]
pub struct DatasetEntry {
    pub entry_type: DatasetEntryType,
    pub id: Strand,
    // This should be bool, so <=0 is false, >0 is true
    pub sampleable: Number,
    pub entry_tags: Vec<DatasetEntryTag>,
}

/// This dictates the type of question and required fields in the DatasetEntry
#[derive(Serialize, Deserialize, Debug)]
pub enum DatasetEntryType {
    None,
    SingleChoice,
    MultipleChoice,
    Category,
}

/// Because a question entry can be formed in multiple ways, this is a collection of various
/// questions and answers and other elements of the entry
///
/// If order matters, then they are ordered as part of the list
#[derive(Serialize, Deserialize, Debug)]
pub enum DatasetEntryTag {
    Question,
    Answer,
    Category,
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
