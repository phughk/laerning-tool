use crate::TerminateMessage;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::style::Stylize;
use ratatui::widgets::Paragraph;
use ratatui::Frame;
use tokio::sync::mpsc::Receiver;
use tokio::sync::watch::Sender;

pub(crate) struct RootComponent {
    pub(crate) terminate_send: Sender<TerminateMessage>,
    pub(crate) input_recv: Receiver<Event>,
}

impl RootComponent {
    pub(crate) fn draw(&mut self, frame: &mut Frame) -> () {
        self.handle_input();
        let area = frame.size();
        frame.render_widget(
            Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                .white()
                .on_blue(),
            area,
        );
    }

    fn handle_input(&mut self) {
        if let Ok(event) = self.input_recv.try_recv() {
            if let event::Event::Key(key) = event {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    // TODO remove unwrap
                    self.terminate_send
                        .send(TerminateMessage::Terminate)
                        .unwrap();
                }
            }
        }
    }
}
