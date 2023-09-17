use crate::repository::LaerningToolRepository;
use crate::xml::LearningModule;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

use surrealdb::sql;

use surrealdb::sql::{Array, Number, Object, Strand, Value};

use super::{Repository, RepositoryError};

/// A dataset represents a collection of information (and questions) that can be used to generate
/// a Game.
///
/// It includes the question and answer pairs, but also dataset meta data.
/// It is not tied to any single user.
#[derive(Serialize, Deserialize, Debug)]
pub struct Dataset {
    // This is not part of the XML
    pub id: Option<sql::Thing>,

    // Below is part of the XML
    pub metadata: DatasetMetadata,
    pub entries: Vec<DatasetEntry>,
}

impl Into<Object> for Dataset {
    fn into(self) -> Object {
        let mut map: BTreeMap<String, Value> = BTreeMap::new();
        map.insert(
            "id".to_string(),
            self.id.map_or(Value::None, |thing| Value::Thing(thing)),
        );
        map.insert("metadata".to_string(), Value::Object(self.metadata.into()));
        map.insert(
            "entries".to_string(),
            Value::Array(Array::from(
                self.entries
                    .into_iter()
                    .map(|dataset_entry| dataset_entry.into())
                    .map(|object| Value::Object(object))
                    .collect::<Vec<Value>>(),
            )),
        );
        Object::from(map)
    }
}

impl From<LearningModule> for Dataset {
    fn from(value: LearningModule) -> Self {
        Dataset {
            id: None,
            metadata: DatasetMetadata {
                name: Strand(value.metadata.name),
                description: Strand(value.metadata.description),
                author: Strand(value.metadata.author),
                updated: Default::default(),
                file_version: Default::default(),
                format_version: Default::default(),
            },
            entries: vec![],
        }
    }
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

impl Into<Object> for DatasetMetadata {
    fn into(self) -> Object {
        let mut map: BTreeMap<String, Value> = BTreeMap::new();
        map.insert("name".to_string(), Value::Strand(self.name));
        map.insert("description".to_string(), Value::Strand(self.description));
        map.insert("author".to_string(), Value::Strand(self.author));
        map.insert("updated".to_string(), Value::Strand(self.updated));
        map.insert("file_version".to_string(), Value::Strand(self.file_version));
        map.insert(
            "format_version".to_string(),
            Value::Strand(self.format_version),
        );
        Object::from(map)
    }
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

impl Into<Object> for DatasetEntry {
    fn into(self) -> Object {
        let mut map: BTreeMap<String, Value> = BTreeMap::new();
        map.insert("entry_type".to_string(), self.entry_type.into());
        map.insert("id".to_string(), Value::Strand(self.id));
        map.insert("sampleable".to_string(), Value::Number(self.sampleable));
        map.insert(
            "entry_tags".to_string(),
            Value::Array(Array::from(
                self.entry_tags
                    .into_iter()
                    .map(|entry_tag| entry_tag.into())
                    .collect::<Vec<Value>>(),
            )),
        );
        Object::from(map)
    }
}

/// This dictates the type of question and required fields in the DatasetEntry
#[derive(Serialize, Deserialize, Debug)]
pub enum DatasetEntryType {
    None,
    SingleChoice,
    MultipleChoice,
    Category,
}

impl Into<Value> for DatasetEntryType {
    fn into(self) -> Value {
        match self {
            DatasetEntryType::None => Value::Number(Number::Int(0)),
            DatasetEntryType::SingleChoice => Value::Number(Number::Int(1)),
            DatasetEntryType::MultipleChoice => Value::Number(Number::Int(2)),
            DatasetEntryType::Category => Value::Number(Number::Int(3)),
        }
    }
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

impl Into<Value> for DatasetEntryTag {
    fn into(self) -> Value {
        match self {
            DatasetEntryTag::Question => Value::Number(Number::Int(0)),
            DatasetEntryTag::Answer => Value::Number(Number::Int(1)),
            DatasetEntryTag::Category => Value::Number(Number::Int(2)),
        }
    }
}

#[derive(Deserialize, Debug)]
pub enum DatasetError {
    CreateDatasetFailed,
}

#[async_trait]
impl Repository<Dataset, DatasetError> for LaerningToolRepository {
    async fn create_dataset(&self, dataset: Dataset) -> Result<Dataset, RepositoryError<DatasetError>> {
        let mut bindings: HashMap<String, Value> = HashMap::new();
        bindings.insert("data".to_string(), Value::Object(dataset.into()));

        let created: Option<Dataset> = self
            .db
            .query("INSERT INTO dataset $data")
            .bind(bindings)
            .await
            .map_err(|_| RepositoryError::CreationFailed(DatasetError::CreateDatasetFailed))
            .unwrap()
            .take(0)
            .unwrap();

        Ok(created.unwrap())
    }

    async fn create_batch_datasets(
        &self,
        datasets: Vec<Dataset>,
    ) -> Result<(), RepositoryError<DatasetError>> {
        for dataset in datasets {
            self.create_dataset(dataset).await.unwrap();
        }
        Ok(())
    }

    async fn create_list(&self) -> Result<Vec<Dataset>, RepositoryError<DatasetError>> {
        let data: Vec<Dataset> = self
            .db
            .query("SELECT * FROM  dataset")
            .await
            .unwrap()
            .take(0)
            .unwrap();

        Ok(data)
    }
}
