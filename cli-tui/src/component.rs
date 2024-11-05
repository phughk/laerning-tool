use ratatui::Frame;

pub trait TerminalComponent {
    fn draw(&mut self, frame: &mut Frame) -> ();
    fn handle_input(&mut self) -> ();
}
