pub struct Dataset {
    pub id: String,
    pub name: String,
    pub author: String,
    pub description: String,
    pub version: String,
    pub questions: Vec<Question>
}

pub struct Question {
    pub id: String,
    pub question_type: QuestionType,
}

pub enum QuestionType {
    Freetext(Box<FreetextQuestion>),
}

pub struct FreetextQuestion {
    pub question: String,
    pub answer: String,
}

