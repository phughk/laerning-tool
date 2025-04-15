use crate::dataset::{Dataset, Question, QuestionType};
use rand::{Rng, RngCore, SeedableRng};
use std::sync::Arc;
use text_distance::DamerauLevenshtein;

pub struct Game {
    pub dataset: Arc<Dataset>,
    pub answered: Vec<AnsweredQuestion>,
    pub rng: rand_chacha::ChaCha12Rng,
    pub current_question_index: usize,
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
    pub fn new(dataset: Arc<Dataset>, seed: Option<u64>) -> Game {
        let seed = seed.unwrap_or(rand::rng().next_u64());
        let mut rng = rand_chacha::ChaCha12Rng::seed_from_u64(seed);
        let answered = Vec::new();
        let current_question = generate_next_question(&mut rng, &dataset);
        Game {
            dataset,
            answered,
            rng,
            current_question_index: current_question,
        }
    }

    pub fn current_question(&self) -> &Question {
        &self.dataset.questions[self.current_question_index]
    }

    pub fn submit_answer(&mut self, answer: String) -> f32 {
        let question = self.current_question();
        let points = match &question.question {
            QuestionType::Freetext(f) => {
                // find a Jaccard similarity that passes threshold
                let mut ret = 0.0;
                let lowercase_provided_answer = answer.to_lowercase();
                for a in &f.answers {
                    let a = a.to_lowercase();
                    // let jaccard =
                    //     stringmetrics::jaccard(a.chars(), lowercase_provided_answer.chars());
                    const OPTIMAL_STRING_ALIGNMENT: bool = true;
                    let similarity = DamerauLevenshtein {
                        src: a,
                        tar: lowercase_provided_answer.clone(),
                        restricted: OPTIMAL_STRING_ALIGNMENT,
                    }
                    .normalized_similarity();
                    if similarity >= f.tolerance as f64 {
                        ret = 1.0;
                    }
                }
                ret
            }
        };
        self.answered.push(AnsweredQuestion {
            question_id: question.id.clone(),
            points,
        });
        if points > 0.0 {
            self.current_question_index = generate_next_question(&mut self.rng, &self.dataset);
        }
        points
    }
}

fn generate_next_question<'a>(rng: &mut rand_chacha::ChaCha12Rng, dataset: &'a Dataset) -> usize {
    let sz = dataset.questions.len();
    let next_id = rng.random_range(0..sz);
    next_id
}
