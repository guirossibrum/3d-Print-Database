use crossterm::{
    cursor::{Hide, Show},
    execute,
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode,
    },
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;

mod api;
mod models;
mod state;
mod handlers;
mod ui;

use state::App;

// Get version from Cargo.toml
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn print_version() {
    println!("3D Print Database TUI (Rust) v{}", VERSION);
}

#[derive(Debug)]
enum TerminalError {
    NotInteractive,
    SetupFailed(Box<dyn std::error::Error>),
}

fn setup_terminal()
-> Result<ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>, TerminalError> {
    // Check if we're in an interactive terminal
    if let Err(e) = enable_raw_mode() {
        if e.raw_os_error() == Some(6) {
            return Err(TerminalError::NotInteractive);
        }
        return Err(TerminalError::SetupFailed(e.into()));
    }

    let mut stdout = io::stdout();
    // Aggressive screen clearing
    execute!(stdout, Clear(ClearType::All)).map_err(|e| TerminalError::SetupFailed(e.into()))?;
    execute!(stdout, Hide).map_err(|e| TerminalError::SetupFailed(e.into()))?;
    execute!(stdout, EnterAlternateScreen).map_err(|e| TerminalError::SetupFailed(e.into()))?;
    // Clear again after entering alternate screen
    execute!(stdout, Clear(ClearType::All)).map_err(|e| TerminalError::SetupFailed(e.into()))?;

    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend).map_err(|e| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
        TerminalError::SetupFailed(e.into())
    })?;

    Ok(terminal)
}

fn print_usage_instructions() {
    println!("✗ No interactive terminal detected!");
    println!();
    println!(
        "This is a Terminal User Interface (TUI) application that requires an interactive terminal."
    );
    println!();
    println!("To run the application:");
    println!("1. Open a terminal/command prompt");
    println!("2. Navigate to the frontend_tui_rust directory");
    println!("3. Run: cargo run");
    println!();
    println!("Make sure the backend is running first:");
    println!(
        "  cd ../backend && python -m uvicorn app.main:app --host 0.0.0.0 --port 8000 --reload"
    );
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Print version
    print_version();

    // Initialize app without println to avoid text persistence
    let mut app = match App::new() {
        Ok(app) => app,
        Err(e) => {
            eprintln!("Failed to initialize app: {:?}", e);
            return Err(e.into());
        }
    };

    let mut terminal = match setup_terminal() {
        Ok(terminal) => terminal,
        Err(TerminalError::NotInteractive) => {
            print_usage_instructions();
            return Ok(());
        }
        Err(TerminalError::SetupFailed(e)) => {
            eprintln!("✗ Failed to setup terminal: {:?}", e);
            return Err(e);
        }
    };

    println!("✓ Terminal setup complete. Starting TUI...");
    println!("Press 'q' to quit, use mouse or keyboard to navigate.");

    // Create app and run it
    let res = app.run(&mut terminal, VERSION);

    // Restore terminal
    let _ = disable_raw_mode();
    let _ = execute!(terminal.backend_mut(), LeaveAlternateScreen);
    let _ = execute!(io::stdout(), Show);

    match res {
        Ok(()) => eprintln!("✓ TUI exited normally."),
        Err(err) => {
            eprintln!("✗ TUI error: {:?}", err);
            return Err(err.into());
        }
    }

    Ok(())
}
