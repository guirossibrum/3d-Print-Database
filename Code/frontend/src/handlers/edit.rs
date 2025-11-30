// handlers/edit.rs - Unified edit mode handlers for product creation/modification

use crate::models::InputMode;
use crate::state::App;
use crossterm::event::KeyEvent;
use anyhow::Result;

/// Field order for navigation
const FIELD_ORDER: &[InputMode] = &[
    InputMode::EditName,
    InputMode::EditDescription,
    InputMode::EditCategory,
    InputMode::EditProduction,
    InputMode::EditTags,
    InputMode::EditMaterials,
];

/// Get next field in circular navigation
fn next_field(current: InputMode) -> InputMode {
    let current_idx = FIELD_ORDER.iter().position(|&m| m == current).unwrap_or(0);
    let next_idx = (current_idx + 1) % FIELD_ORDER.len();
    FIELD_ORDER[next_idx]
}

/// Get previous field in circular navigation
fn prev_field(current: InputMode) -> InputMode {
    let current_idx = FIELD_ORDER.iter().position(|&m| m == current).unwrap_or(0);
    let prev_idx = if current_idx == 0 { FIELD_ORDER.len() - 1 } else { current_idx - 1 };
    FIELD_ORDER[prev_idx]
}

/// Handle edit mode events (unified for create and edit)
pub fn handle(app: &mut App, key: KeyEvent) -> Result<()> {
    use crossterm::event::KeyCode;

    match key.code {
        // ESC to cancel and return to normal mode
        KeyCode::Esc => {
            app.clear_new_product(); // Clear any new product
            app.set_input_mode(InputMode::Normal);
            app.set_status("Edit cancelled".to_string());
        }

        // ENTER to save changes
        KeyCode::Enter => {
            // TODO: Implement save functionality (create vs update)
            app.clear_new_product(); // Clear new product after save
            app.set_input_mode(InputMode::Normal);
            app.set_status("Product saved".to_string());
        }

        // TAB or Down to next field
        KeyCode::Tab | KeyCode::Down => {
            let next_mode = next_field(app.input_mode());
            app.set_input_mode(next_mode);
            app.set_status(format!("Editing {:?}", next_mode));
        }

        // Up to previous field
        KeyCode::Up => {
            let prev_mode = prev_field(app.input_mode());
            app.set_input_mode(prev_mode);
            app.set_status(format!("Editing {:?}", prev_mode));
        }

        // Character input for text fields
        KeyCode::Char(c) => {
            let mode = app.input_mode();
            if let Some(product) = app.get_current_product_mut() {
                match mode {
                    InputMode::EditName => {
                        product.name.push(c);
                    }
                    InputMode::EditDescription => {
                        product.description.get_or_insert_with(String::new).push(c);
                    }
                    _ => {} // Other fields handled via selection
                }
                app.set_status(format!("Input: {}", c));
            }
        }

        // Backspace for text deletion
        KeyCode::Backspace => {
            let mode = app.input_mode();
            if let Some(product) = app.get_current_product_mut() {
                match mode {
                    InputMode::EditName => {
                        product.name.pop();
                    }
                    InputMode::EditDescription => {
                        if let Some(desc) = &mut product.description {
                            desc.pop();
                        }
                    }
                    _ => {}
                }
            }
        }

        _ => {}
    }

    Ok(())
}