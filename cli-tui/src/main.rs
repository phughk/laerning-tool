use std::error::Error;
use std::io::{stdout, Stdout};
use std::panic::catch_unwind;
use std::time::Duration;

use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{event, ExecutableCommand};
use log::trace;
use ratatui::backend::CrosstermBackend;
use ratatui::style::Stylize;
use ratatui::Terminal;
use tokio::sync::mpsc::Sender;
use tokio::sync::watch::Receiver;

use crate::root::RootComponent;

mod component;
mod root;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(debug_assertions)]
    {
        let port = 6669;
        println!("Debug mode enabled.");
        println!("You can set `RUSTFLAGS=\"--cfg tokio_unstable\" to enable tokio-console access. Using this forces a recompile of all dependencies.");
        println!("That can be avoided with `cargo rustc -- --cfg tokio_unstable`");
        let a = catch_unwind(|| {
            console_subscriber::ConsoleLayer::builder()
                .server_addr(([127, 0, 0, 1], port))
                .init();
        });
        match a {
            Ok(_) => println!("Console subscriber initialized on port {port}."),
            Err(_) => println!("Console subscriber failed to initialize"),
        }

        tui_logger::init_logger(log::LevelFilter::Trace).unwrap();

        tokio::time::sleep(Duration::from_secs(2)).await;
    }
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let (terminate_send, terminate_recv) = tokio::sync::watch::channel(TerminateMessage::Live);
    let (input_send, input_recv) = tokio::sync::mpsc::channel::<event::Event>(20);
    trace!("Spawning input handle");
    let input_handle = tokio::spawn(read_input(terminate_recv.clone(), input_send));
    let root = RootComponent {
        terminate_send,
        input_recv,
    };
    trace!("Spawning root handle");
    let root_handle = tokio::spawn(draw_root(terminal, terminate_recv, root));

    trace!("Waiting for handles to finish");
    let (input_handle_res, root_handle_res) = tokio::join!(input_handle, root_handle);
    // Return screen
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    // Final error handling
    input_handle_res?.unwrap();
    root_handle_res?.unwrap();
    Ok(())
}

async fn draw_root(
    mut terminal: Terminal<CrosstermBackend<Stdout>>,
    mut terminate_recv: Receiver<TerminateMessage>,
    mut root: RootComponent,
) -> Result<(), AppError> {
    loop {
        trace!("New draw loop");
        if *terminate_recv.borrow() == TerminateMessage::Terminate {
            break;
        }
        terminal
            .draw(|f| root.draw(f))
            .map_err(|_| AppError::CompletedFrame)?;
        tokio::task::yield_now().await;
    }
    Ok(())
}

async fn read_input(
    mut receiver: Receiver<TerminateMessage>,
    input_send: Sender<event::Event>,
) -> Result<(), ReadInputError> {
    loop {
        trace!("New input loop");
        if *receiver.borrow() == TerminateMessage::Terminate {
            break;
        }
        if event::poll(std::time::Duration::from_millis(1))? {
            let read_event = event::read()?;
            input_send
                .send(read_event)
                .await
                .map_err(|_| ReadInputError::SendError)?;
        }
        tokio::task::yield_now().await;
    }
    Ok(())
}

#[derive(Debug, PartialEq)]
enum TerminateMessage {
    Live,
    Terminate,
}

#[derive(Debug)]
enum AppError {
    CompletedFrame,
}

#[derive(Debug)]
enum ReadInputError {
    Io(std::io::Error),
    SendError,
    // Crossterm(crossterm::ErrorKind),
}

impl From<std::io::Error> for ReadInputError {
    fn from(e: std::io::Error) -> Self {
        ReadInputError::Io(e)
    }
}
