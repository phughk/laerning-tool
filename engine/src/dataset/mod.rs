#[cfg(test)]
mod test;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Dataset {
    pub id: String,
    pub name: String,
    pub author: String,
    pub description: String,
    pub version: String,
    pub questions: Vec<Question>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Question {
    pub id: String,
    pub question: QuestionType,
}

/// Question types are actually matchers, hence matchers are not included as separate struct
/// and question types can overlap
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum QuestionType {
    /// Allows freetext typing one of several valid answers with configured leniency
    Freetext(Box<FreetextQuestion>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FreetextQuestion {
    pub question_prompt: String,
    /// An array of valid answers, useful for example if a word means several things or can have
    /// different response. Ex "Synonym for Welcome?" could be ["Hello", "Hi", "Greetings"] etc
    pub answers: Vec<String>,
    /// Jaccard similarity threshold (allowing for typos, and capitalisation errors)
    /// 1.0 means identical, 0.0 means no shared characters in the answers
    pub tolerance: f32,
}
