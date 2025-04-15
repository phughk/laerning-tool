use egui::{Color32, Rect, Response, Ui, Vec2, Widget};
use std::time::{Duration, Instant};

pub struct TimeoutBarWidget {
    pub start_time: Instant,
    pub total_duration: Duration,
    pub height: f32,
    pub color: Color32,
}

impl TimeoutBarWidget {
    pub fn new(total_duration: Duration) -> Self {
        Self {
            start_time: Instant::now() - total_duration,
            total_duration,
            height: 4.0,
            color: Color32::LIGHT_GREEN,
        }
    }

    /// Optional: reset timer
    pub fn reset(&mut self, color32: Color32) {
        self.start_time = Instant::now();
        self.color = color32;
    }

    /// Returns the current progress (1.0 → 0.0)
    pub fn progress(&self) -> f32 {
        let elapsed = self.start_time.elapsed();
        let total = self.total_duration.as_secs_f32();
        let elapsed_secs = elapsed.as_secs_f32();
        (1.0 - (elapsed_secs / total)).clamp(0.0, 1.0)
    }
}

impl Widget for &mut TimeoutBarWidget {
    fn ui(self, ui: &mut Ui) -> Response {
        let desired_size = Vec2::new(ui.available_width(), self.height);
        let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::hover());

        let progress = self.progress();
        let filled_width = rect.width() * progress;

        let filled_rect = Rect::from_min_size(rect.min, Vec2::new(filled_width, self.height));

        let visuals = ui.style().visuals.clone();
        let bg_color = visuals.extreme_bg_color; // optional background color

        ui.painter().rect_filled(rect, 0.0, bg_color); // background
        ui.painter().rect_filled(filled_rect, 0.0, self.color); // progress

        response
    }
}
