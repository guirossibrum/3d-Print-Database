// src/handlers/navigation.rs
use anyhow::Result;
use crossterm::event::KeyEvent;
use crate::app::App;

pub fn handle(app: &mut App, key: KeyEvent) -> Result<bool> {
    use crossterm::event::KeyCode;

    // These are "global" navigation keys handled when none of the more specific modules handled the event.
    match key.code {
        KeyCode::Char('n') => {
            // start create flow if in Create tab
            if matches!(app.current_tab, crate::state::Tab::Create) {
                app.input_mode = crate::state::InputMode::CreateName;
                app.active_pane = crate::state::ActivePane::Left;
                return Ok(true);
            }
        }
        KeyCode::Esc => {
            // Global cancel: return to Normal mode
            app.input_mode = crate::state::InputMode::Normal;
            app.active_pane = crate::state::ActivePane::Left;
            return Ok(true);
        }
        _ => {}
    }

    Ok(false)
}
