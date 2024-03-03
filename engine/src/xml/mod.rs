pub(crate) mod error;
#[cfg(test)]
mod module_browser_test;

use scan_fmt::scan_fmt;
use serde::de;
use serde::de::Visitor;
use serde::Deserialize;
use serde::Serialize;
use serde_xml_rs::de::Deserializer;
use std::fmt::Formatter;
use std::fs;
use std::io;

use xml::reader::EventReader;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[allow(dead_code)]
pub struct LearningModule {
    pub metadata: LearningModuleMetadata,
    pub entries: LearningModuleEntries,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[allow(dead_code)]
pub struct LearningModuleEntries {
    #[serde(rename = "$value")]
    pub entries: Vec<LearningModuleEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[allow(dead_code)]
#[serde(rename_all = "kebab-case")]
pub struct LearningModuleMetadata {
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub author: String,
    pub updated: String,
    pub file_version: Version,
    pub format_version: Version,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[allow(dead_code)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

struct VersionVisitor {}

impl<'de> Visitor<'de> for VersionVisitor {
    type Value = Version;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("expecting version format <major>.<minor>.<patch> strictly")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if let Ok((major, minor, patch)) = scan_fmt!(&v, "{d}.{d}.{d}", u32, u32, u32) {
            return Ok(Version {
                major,
                minor,
                patch,
            });
        }
        Err(de::Error::custom("not a version"))
    }

    /*
        TODO: #38 you should not implement `visit_string` without also implementing `visit_str`,
        hence `visit_str` has to be implemented
    */
    fn visit_str<E>(self, _v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        todo!()
    }
}

impl<'de> de::Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_string(VersionVisitor {})
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[allow(dead_code)]
pub struct LearningModuleEntry {
    #[serde(rename = "type", default)]
    pub entry_type: LearningModuleEntryType,
    pub id: String,
    #[serde(default)]
    pub sampleable: bool,
    #[serde(rename = "$value")]
    pub entry_tags: Vec<EntryTag>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[allow(dead_code)]
#[serde(rename_all = "kebab-case")]
pub enum EntryTag {
    Question(Question),
    Answer(Answer),
    CategoryDeclaration(CategoryDeclaration),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[allow(dead_code)]
pub struct Question {
    #[serde(rename = "$value")]
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[allow(dead_code)]
pub struct Answer {
    #[serde(default)]
    pub correct: bool,
    pub id: String,
    #[serde(default)]
    pub category: String,
    #[serde(rename = "$value")]
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[allow(dead_code)]
pub struct CategoryDeclaration {
    pub id: String,
    #[serde(rename = "$value")]
    pub label: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum LearningModuleEntryType {
    #[default]
    None,
    SingleChoice,
    MultipleChoice,
    Category,
}

pub fn list_modules(directory: &str) -> Result<Vec<LearningModule>, error::Error> {
    let paths = fs::read_dir(directory).map_err(|_e| -> error::Error { error::Error::Io {} })?;
    let mut ret = Vec::new();
    for path in paths {
        let module = read_module(path.as_ref().unwrap().path().display().to_string());
        match module {
            Ok(_) => ret.push(module?),
            Err(e) => {
                let e_str = format!("{e:?}");
                let dir_e =
                    path.map_err(|_std_io_error| -> error::Error { error::Error::Io {} })?;
                let path_str = format!("{dir_e:?}");
                let mod_err = error::Error::ListModule {
                    error: e_str,
                    path: path_str,
                };
                return Err(mod_err);
            }
        }
    }
    Ok(ret)
}

fn read_module(filename: String) -> Result<LearningModule, error::Error> {
    let file = fs::File::open(filename).unwrap();
    let file = io::BufReader::new(file);
    let reader = EventReader::new(file);
    read_module_content(reader)
}

fn read_module_content(
    event_reader: EventReader<io::BufReader<fs::File>>,
) -> Result<LearningModule, error::Error> {
    match LearningModule::deserialize(&mut Deserializer::new(event_reader)) {
        Ok(x) => Ok(x),
        Err(_) => Err(error::Error::Serde {}),
    }
}
