mod api;
mod models;
mod state;
mod handlers;
mod ui;

use anyhow::Result;
use crossterm::{
    cursor::{Hide, Show},
    event::{self},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType, SetTitle},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io;

#[derive(Debug)]
enum TerminalError {
    NotInteractive,
    SetupFailed(String),
}

impl From<std::io::Error> for TerminalError {
    fn from(error: std::io::Error) -> Self {
        TerminalError::SetupFailed(error.to_string())
    }
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>, TerminalError> {
    // Check interactive terminal
    if let Err(e) = enable_raw_mode() {
        if e.raw_os_error() == Some(6) {
            if std::env::var("XDG_TERMINAL_TTY").is_ok()
                || std::env::var("GHOSTTY_RESOURCES_DIR").is_ok()
            {
                // Continue for terminal emulators
            } else {
                return Err(TerminalError::NotInteractive);
            }
        }
        return Err(TerminalError::SetupFailed(e.to_string()));
    }

    let mut stdout = io::stdout();
    execute!(stdout, SetTitle("3D Print Database TUI"))?;
    execute!(stdout, Clear(ClearType::All))?;
    execute!(stdout, Hide)?;
    execute!(stdout, EnterAlternateScreen)?;
    execute!(stdout, Clear(ClearType::All))?;

    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend).map_err(|e| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
        TerminalError::SetupFailed(e.to_string())
    })?;

    Ok(terminal)
}

fn print_usage_instructions() {
    println!("✗ No interactive terminal detected!");
    println!();
    println!("This is a Terminal User Interface (TUI) application that requires an interactive terminal.");
    println!();
    println!("To run application:");
    println!("1. Open a terminal/command prompt");
    println!("2. Navigate to frontend directory");
    println!("3. Run: cargo run");
    println!();
    println!("Make sure backend is running first:");
    println!(
        "  cd ../backend && python -m uvicorn app.main:app --host 0.0.0.0 --port 8000 --reload"
    );
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize app
    let mut app = state::App::new()?;
    app.load_all_data().await?;

    // Setup terminal with error handling
    let mut terminal = match setup_terminal() {
        Ok(terminal) => terminal,
        Err(TerminalError::NotInteractive) => {
            print_usage_instructions();
            return Ok(());
        }
        Err(TerminalError::SetupFailed(e)) => {
            eprintln!("✗ Failed to setup terminal: {:?}", e);
            return Err(anyhow::anyhow!("Terminal setup failed: {}", e));
        }
    };

    // Main application loop
    let res = (|| -> Result<()> {
        while app.running {
            // Draw UI
            terminal.draw(|f| {
                ui::draw(f, &app);
            })?;

            // Handle events
            if event::poll(std::time::Duration::from_millis(100))? {
                let event = event::read()?;
                handlers::handle_event(&mut app, event)?;
            }
        }
        Ok(())
    })();

    // Cleanup terminal
    let _ = disable_raw_mode();
    let _ = execute!(terminal.backend_mut(), LeaveAlternateScreen);
    let _ = execute!(io::stdout(), Show);

    match res {
        Ok(()) => println!("✓ TUI exited normally."),
        Err(err) => {
            eprintln!("✗ TUI error: {:?}", err);
            return Err(err);
        }
    }

    Ok(())
}