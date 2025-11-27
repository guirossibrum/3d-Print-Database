// src/handlers/create/create_materials.rs
//! Handle material input and selection during creation

use anyhow::Result;
use crossterm::event::KeyEvent;

use crate::App;

pub fn handle(app: &mut App, key: KeyEvent) -> Result<bool> {
    use crossterm::event::KeyCode;

    match app.input_mode {
        crate::models::InputMode::CreateMaterials => {
            match key.code {
                KeyCode::Esc => {
                    app.input_mode = crate::models::InputMode::Normal;
                    app.create_form.materials.clear();
                    app.create_form = crate::models::CreateForm {
                        production: true, // Reset to default
                        ..Default::default()
                    };
                    app.active_pane = crate::models::ActivePane::Left;
                }
                KeyCode::Enter => {
                    // Save product
                    app.save_product()?;
                    app.input_mode = crate::models::InputMode::Normal;
                    app.active_pane = crate::models::ActivePane::Left;
                }
                KeyCode::Tab => {
                    app.tag_selection = vec![false; app.materials.len()];
                    // Pre-select materials that are already in create_form.materials
                    for (i, material) in app.materials.iter().enumerate() {
                        if app.create_form.materials.contains(material) {
                            app.tag_selection[i] = true;
                        }
                    }
                    app.create_form.material_selected_index = 0; // Initialize selection index
                    app.tag_select_mode = crate::models::TagSelectMode::Create;
                    app.item_type = crate::models::ItemType::Material;
                    app.input_mode = crate::models::InputMode::CreateMaterialSelect;
                    app.active_pane = crate::models::ActivePane::Right;
                }
                KeyCode::Up => {
                    app.input_mode = crate::models::InputMode::CreateTags;
                }
                KeyCode::Down => {
                    // Circular navigation: Materials → Name
                    app.input_mode = crate::models::InputMode::CreateName;
                }
                _ => {}
            }
            Ok(true)
        }
        _ => Ok(false),
    }
}