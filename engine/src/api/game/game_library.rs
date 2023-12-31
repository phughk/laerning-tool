use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, ToSchema, Debug, PartialEq)]
pub struct NewGameRequest {
    pub dataset: String,
    pub name: Option<String>,
}

#[derive(Deserialize, Serialize, ToSchema, Debug, PartialEq)]
pub struct GameJson {
    pub name: String,
    pub dataset: String,
    pub current_question: Option<QuestionEntry>,
    pub stats: GameStats,
}

#[derive(Deserialize, Serialize, ToSchema, Debug, PartialEq)]
pub struct GameListing {
    pub name: String,
    pub dataset: String,
    pub started: String,
    pub status: GameStatus,
}

#[derive(Deserialize, Serialize, ToSchema, Debug, PartialEq)]
pub struct QuestionEntry {
    pub question: String,
    pub answer_type: AnswerType,
    pub available_answers: Vec<String>,
}

#[derive(Deserialize, Serialize, ToSchema, Debug, PartialEq)]
pub enum AnswerType {
    SingleChoice,
    MultipleChoice,
    FreeText,
}

#[derive(Deserialize, Serialize, ToSchema, Debug, PartialEq)]
pub struct GameStats {
    pub current_question: u8,
    pub total_questions: u8,
    pub current_try: u8,
    pub max_tries: u8,
    pub duration: u8,
    pub average_question_duration: f32,
}

#[derive(Deserialize, Serialize, ToSchema, Debug, PartialEq)]
pub enum GameStatus {
    Pending,
    InProgress,
    Completed,
}
