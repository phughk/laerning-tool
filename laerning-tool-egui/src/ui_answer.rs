use egui::{Id, Key, Ui};

pub fn draw_answer(ui: &mut Ui, answer_prompt: &mut String) -> Option<String> {
    ui.label("Answer:");
    let answer_box_id = Id::new("answer_box");
    let answer_box = egui::TextEdit::singleline(answer_prompt).id(answer_box_id);
    let response = ui.add(answer_box);
    if response.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
        let answer = answer_prompt.clone();
        answer_prompt.clear();
        ui.memory_mut(|m| m.request_focus(answer_box_id));
        Some(answer)
    } else {
        None
    }
}
