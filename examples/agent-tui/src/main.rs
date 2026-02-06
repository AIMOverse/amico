//! Agent TUI — terminal user interface for the local agent daemon.
//!
//! This TUI connects to the agent daemon over HTTP and provides:
//! - Session listing, creation, and switching
//! - Chat message input and display
//! - Real-time SSE streaming of agent responses
//!
//! ## Key bindings
//!
//! | Key        | Action                           |
//! |------------|----------------------------------|
//! | Tab        | Switch between sessions panel and chat |
//! | Ctrl+N     | Create a new session             |
//! | ↑/↓        | Navigate sessions / scroll chat  |
//! | Enter      | Select session / send message    |
//! | Esc        | Quit                             |

mod api;
mod app;
mod ui;

use app::App;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = std::env::var("AGENT_URL").unwrap_or_else(|_| "http://127.0.0.1:3000".into());

    // Terminal setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run application
    let result = run_app(&mut terminal, &base_url).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(e) = result {
        eprintln!("Error: {e}");
    }

    Ok(())
}

async fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    base_url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new(base_url);

    // Initial session load
    app.refresh_sessions().await;

    loop {
        // Draw UI
        terminal.draw(|f| ui::draw(f, &app))?;

        // Poll for events (with a short timeout so SSE updates render)
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => break,
                    KeyCode::Tab => app.toggle_focus(),
                    KeyCode::Char('n') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.create_session().await;
                    }
                    KeyCode::Up => app.on_up(),
                    KeyCode::Down => app.on_down(),
                    KeyCode::Enter => {
                        app.on_enter().await;
                    }
                    KeyCode::Char(c) => {
                        if app.is_chat_focused() {
                            app.input.push(c);
                        }
                    }
                    KeyCode::Backspace => {
                        if app.is_chat_focused() {
                            app.input.pop();
                        }
                    }
                    _ => {}
                }
            }
        }

        // Drain any SSE updates
        app.poll_stream().await;
    }

    Ok(())
}
