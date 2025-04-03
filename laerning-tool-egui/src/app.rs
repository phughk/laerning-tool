use eframe::{CreationContext, Frame};
use egui::Context;
use laerning_tool::dataset::Dataset;
use laerning_tool::game::Game;

pub struct App {
    pub dataset: Option<Dataset>,
    pub game: Option<Game<'static>>,
}

impl App {
    pub fn new(_cc: CreationContext) -> Self {
        Self {
            dataset: None,
            game: None,
        }
    }

    pub fn update_load_dataset(&mut self, ctx: &Context, frame: &mut Frame) {

    }

    pub fn update_configure_game(&mut self, ctx: &Context, frame: &mut Frame) {

    }

    pub fn update_game(&mut self, ctx: &Context, frame: &mut Frame) {

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