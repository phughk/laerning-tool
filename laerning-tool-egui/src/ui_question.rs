use egui::Ui;

pub fn draw_question(ui: &mut Ui, prompt: &str) {
    ui.label(prompt);
}
