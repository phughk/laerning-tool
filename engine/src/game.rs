use rand::{Rng, RngCore, SeedableRng};
use crate::dataset::{Dataset, Question, QuestionType};

pub struct Game<'a> {
    pub dataset: Dataset,
    pub answered: Vec<AnsweredQuestion>,
    pub rng: rand_chacha::ChaCha12Rng,
    pub current_question: &'a Question,
}

pub enum QuizStrategy {
    Random,
    UntilPoints,
}

pub struct AnsweredQuestion {
    pub question_id: String,
    pub points: f32,
}

impl Game {
    pub fn new(dataset: Dataset, seed: Option<u64>) -> Game {
        let seed = seed.unwrap_or(rand::rng().next_u64());
        let mut rng = rand_chacha::ChaCha12Rng::seed_from_u64(seed);
        let answered = Vec::new();
        let current_question = generate_next_question(&mut rng, &dataset);
        Game {
            dataset,
            answered,
            rng,
            current_question,
        }
    }

    pub fn current_question(&self) -> &Question {
        self.current_question
    }

    pub fn submit_answer(&mut self, answer: String) -> f32 {
        let points = match &self.current_question.question_type {
            QuestionType::Freetext(f) => {
                if f.answer == answer {
                    1.0
                } else {
                    0.0
                }
            }
        };
        self.answered.push(AnsweredQuestion {
            question_id: self.current_question.id.clone(),
            points,
        });
        if points > 0.0 {
            self.current_question = generate_next_question(&mut self.rng, &self.dataset);
        }
        points
    }
}

fn generate_next_question<'a>(rng: &mut rand_chacha::ChaCha12Rng, dataset: &'a Dataset) -> &'a Question {
    let sz = dataset.questions.len();
    let next_id = rng.random_range(0..sz);
    &dataset.questions[next_id]
}
