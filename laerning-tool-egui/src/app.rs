use crate::timeout_widget::TimeoutBarWidget;
use crate::ui_answer::draw_answer;
use crate::ui_question::draw_question;
use eframe::{CreationContext, Frame};
use egui::{Color32, Context, Widget};
use egui_extras::{Size, StripBuilder};
use laerning_tool::dataset::{Dataset, FreetextQuestion, Question, QuestionType};
use laerning_tool::game::{AnswerResult, Game};
use std::sync::Arc;
use std::time::Duration;

pub struct App {
    pub dataset: Option<Arc<Dataset>>,
    pub game: Option<Game>,
    pub answer_prompt: String,
    pub progress_bar: TimeoutBarWidget,
}

impl App {
    pub fn new(_cc: &CreationContext) -> Self {
        let dataset = Dataset {
            id: "".to_string(),
            name: "".to_string(),
            author: "".to_string(),
            description: "".to_string(),
            version: "".to_string(),
            questions: vec![
                Question {
                    id: "abc-123".to_string(),
                    question: QuestionType::Freetext(Box::new(FreetextQuestion {
                        question_prompt: "What is my name".to_string(),
                        answers: vec!["You".to_string()],
                        tolerance: 0.9,
                    })),
                },
                Question {
                    id: "def-456".to_string(),
                    question: QuestionType::Freetext(Box::new(FreetextQuestion {
                        question_prompt: "What is your name".to_string(),
                        answers: vec!["Me".to_string()],
                        tolerance: 0.9,
                    })),
                },
            ],
        };
        let dataset = Arc::new(dataset);
        let game = Game::new(dataset.clone(), None);
        let mut app = Self {
            dataset: Some(dataset),
            game: Some(game),
            answer_prompt: String::with_capacity(1024),
            progress_bar: TimeoutBarWidget::new(Duration::from_secs(5)),
        };
        app
    }

    pub fn update_load_dataset(&mut self, ctx: &Context, _frame: &mut Frame) {
        unimplemented!()
    }

    pub fn update_configure_game(&mut self, ctx: &Context, frame: &mut Frame) {
        unimplemented!()
    }

    pub fn update_game(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            StripBuilder::new(ui)
                .size(Size::initial(32.0))
                .size(Size::remainder())
                .size(Size::initial(32.0))
                .vertical(|mut strip| {
                    strip.cell(|ui| {
                        draw_question(ui, self.game.as_ref().unwrap().current_question().prompt())
                    });
                    strip.cell(|ui| {
                        if let Some(answer) = draw_answer(ui, &mut self.answer_prompt) {
                            match self.game.as_mut().unwrap().submit_answer(answer) {
                                AnswerResult::Correct(_) => {
                                    self.progress_bar.reset(Color32::LIGHT_GREEN)
                                }
                                AnswerResult::Incorrect(_) => {
                                    self.progress_bar.reset(Color32::LIGHT_RED)
                                }
                            }
                        }
                    });
                    strip.cell(|ui| {
                        self.progress_bar.ui(ui);
                    })
                })
        });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        if self.dataset.is_none() {
            self.update_load_dataset(ctx, frame);
        } else if self.game.is_none() {
            self.update_configure_game(ctx, frame);
        } else {
            self.update_game(ctx, frame);
        }
    }
}
