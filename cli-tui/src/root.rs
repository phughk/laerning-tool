use crate::TerminateMessage;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::layout::{Direction, Layout, Rect};
use ratatui::prelude::Constraint;
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, Paragraph};
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

        let root_layout: [Rect; 2] = Layout::new(
            Direction::Vertical,
            [Constraint::Fill(1), Constraint::Fill(1)],
        )
        .areas(area);

        #[cfg(debug_assertions)]
        let root_layout: [Rect; 3] = Layout::new(
            Direction::Vertical,
            [Constraint::Fill(1), Constraint::Fill(1), Constraint::Min(5)],
        )
        .areas(area);

        frame.render_widget(
            Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                .white()
                .on_blue(),
            root_layout[0],
        );
        frame.render_widget(
            Paragraph::new("Press 'q' to quit").white().on_blue(),
            root_layout[1],
        );

        #[cfg(debug_assertions)]
        frame.render_widget(
            tui_logger::TuiLoggerWidget::default().block(
                Block::default()
                    .title("Debug build logs")
                    .borders(Borders::ALL),
            ),
            root_layout[2],
        )
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
