use eframe::{CreationContext, Frame};
use egui::Context;
use egui_extras::{Size, StripBuilder};
use laerning_tool::dataset::{Dataset, FreetextQuestion, Question, QuestionType};
use laerning_tool::game::Game;
use std::sync::Arc;

pub struct App {
    pub dataset: Option<Arc<Dataset>>,
    pub game: Option<Game>,
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
                    question_type: QuestionType::Freetext(Box::new(FreetextQuestion {
                        question: "What is my name".to_string(),
                        answer: "You".to_string(),
                    })),
                },
                Question {
                    id: "def-456".to_string(),
                    question_type: QuestionType::Freetext(Box::new(FreetextQuestion {
                        question: "What is your name".to_string(),
                        answer: "Me".to_string(),
                    })),
                },
            ],
        };
        let dataset = Arc::new(dataset);
        let game = Game::new(dataset.clone(), None);
        let mut app = Self {
            dataset: Some(dataset),
            game: Some(game),
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
                .vertical(|mut strip| {
                    strip.cell(|ui| {
                        ui.label("Question goes here");
                    });
                    strip.cell(|ui| {
                        ui.label("Answer goes here");
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
