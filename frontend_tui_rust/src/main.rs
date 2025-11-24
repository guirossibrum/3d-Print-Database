use std::io;
use ratatui::{backend::CrosstermBackend, Terminal};
use crossterm::{
    cursor::{Hide, Show},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType},
};

mod app;
mod ui;
mod api;

use app::App;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize app without println to avoid text persistence
    let mut app = match App::new().await {
        Ok(app) => app,
        Err(e) => {
            eprintln!("Failed to initialize app: {:?}", e);
            return Err(e.into());
        }
    };

    // Setup terminal with error handling
    if let Err(e) = enable_raw_mode() {
        // Check if this is the "No such device or address" error (code 6)
        // This typically means we're not in an interactive terminal
        if e.raw_os_error() == Some(6) {
            println!("✗ No interactive terminal detected!");
            println!("");
            println!("This is a Terminal User Interface (TUI) application that requires an interactive terminal.");
            println!("");
            println!("To run the application:");
            println!("1. Open a terminal/command prompt");
            println!("2. Navigate to the frontend_tui_rust directory");
            println!("3. Run: cargo run");
            println!("");
            println!("Make sure the backend is running first:");
            println!("  cd ../backend && python -m uvicorn app.main:app --host 0.0.0.0 --port 8000 --reload");
            return Ok(());
        }
        println!("✗ Failed to enable raw mode: {:?}", e);
        return Err(e.into());
    }

    let mut stdout = io::stdout();
    // Aggressive screen clearing
    execute!(stdout, Clear(ClearType::All))?;
    execute!(stdout, Hide)?;
    execute!(stdout, EnterAlternateScreen)?;
    // Clear again after entering alternate screen
    execute!(stdout, Clear(ClearType::All))?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = match Terminal::new(backend) {
        Ok(t) => t,
        Err(e) => {
            println!("✗ Failed to create terminal: {:?}", e);
            let _ = disable_raw_mode();
            let _ = execute!(io::stdout(), LeaveAlternateScreen);
            return Err(e.into());
        }
    };

    println!("✓ Terminal setup complete. Starting TUI...");
    println!("Press 'q' to quit, use mouse or keyboard to navigate.");

    // Create app and run it
    let res = app.run(&mut terminal).await;

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
