use anyhow::Result;
use crossterm::event::KeyCode;

use crate::models::*;

/// Dispatch key events to appropriate handler functions
pub fn handle_key_dispatch(app: &mut super::App, key: crossterm::event::KeyEvent) -> Result<()> {
    match app.input_mode {
        InputMode::Normal => handle_normal_mode(app, key),
        InputMode::Search => handle_search_mode(app, key),
        InputMode::InventorySearch => handle_inventory_search_mode(app, key),
        InputMode::CreateName => handle_create_name_mode(app, key),
        InputMode::CreateDescription => handle_create_description_mode(app, key),
        InputMode::CreateCategory => handle_create_category_mode(app, key),
        InputMode::CreateCategorySelect => handle_create_category_select_mode(app, key),
        InputMode::CreateProduction => handle_create_production_mode(app, key),
        InputMode::CreateTags => handle_create_tags_mode(app, key),
        InputMode::CreateTagSelect => handle_tag_select_mode(app, key),
        InputMode::EditName => handle_edit_name_mode(app, key),
        InputMode::EditDescription => handle_edit_description_mode(app, key),
        InputMode::EditProduction => handle_edit_production_mode(app, key),
        InputMode::EditTags => handle_edit_tags_mode(app, key),
        InputMode::EditTagSelect => handle_tag_select_mode(app, key),
        InputMode::NewTag | InputMode::NewCategory => handle_new_item_mode(app, key),
        InputMode::EditTag | InputMode::EditCategory => handle_edit_item_mode(app, key),
    }
}

fn handle_normal_mode(app: &mut super::App, key: crossterm::event::KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => {
            app.running = false;
        }
        KeyCode::Tab => {
            if app.has_multiple_panes()
                && matches!(app.active_pane, ActivePane::Left)
                && !app.products.is_empty()
            {
                // Refresh data before editing
                app.refresh_data();
                // Backup current product for potential cancellation
                if let Some(product) = app.products.get(app.selected_index) {
                    app.edit_backup = Some(product.clone());
                }
                // Initialize edit_tags_string with current product tags
                if let Some(product) = app.products.get(app.selected_index) {
                    app.edit_tags_string = product.tags.join(", ");
                }
                // Switch to right pane and enter edit mode
                app.active_pane = ActivePane::Right;
                app.input_mode = InputMode::EditName;
            } else if app.has_multiple_panes() {
                // Regular pane switching
                app.next_pane();
            }
        }
        KeyCode::BackTab if app.has_multiple_panes() => {
            app.prev_pane();
        }
        KeyCode::BackTab => {
            app.current_tab = app.current_tab.prev();
            app.active_pane = ActivePane::Left;
            app.selected_index = 0;
            app.filtered_selection_index = 0;
        }
        KeyCode::Down | KeyCode::Char('j') => {
            // Use filtered navigation if in search/inventory tabs with active search
            if (matches!(app.current_tab, Tab::Search) && !app.search_query.is_empty()) ||
               (matches!(app.current_tab, Tab::Inventory) && !app.inventory_search_query.is_empty()) {
                app.next_filtered_item();
            } else {
                app.next_item();
            }
        }
        KeyCode::Up | KeyCode::Char('k') => {
            // Use filtered navigation if in search/inventory tabs with active search
            if (matches!(app.current_tab, Tab::Search) && !app.search_query.is_empty()) ||
               (matches!(app.current_tab, Tab::Inventory) && !app.inventory_search_query.is_empty()) {
                app.prev_filtered_item();
            } else {
                app.prev_item();
            }
        }
        KeyCode::Left => {
            app.current_tab = app.current_tab.prev();
            app.active_pane = ActivePane::Left;
            app.selected_index = 0;
            app.filtered_selection_index = 0;
            app.refresh_data();
        }
        KeyCode::Right => {
            app.current_tab = app.current_tab.next();
            app.active_pane = ActivePane::Left;
            app.selected_index = 0;
            app.filtered_selection_index = 0;
            app.refresh_data();
        }
        KeyCode::Char('/') => {
            if matches!(app.current_tab, Tab::Search) {
                app.input_mode = InputMode::Search;
            } else if matches!(app.current_tab, Tab::Inventory) {
                app.input_mode = InputMode::InventorySearch;
            }
        }
        KeyCode::Enter => {
            match app.input_mode {
                InputMode::Normal => {
                    if matches!(app.current_tab, Tab::Create) {
                        app.input_mode = InputMode::CreateName;
                    } else if matches!(app.current_tab, Tab::Search)
                        && !app.products.is_empty()
                    {
                        // Direct edit from normal mode (legacy behavior)
                        app.input_mode = InputMode::EditName;
                    }
                }
                InputMode::EditName
                    | InputMode::EditDescription
                    | InputMode::EditProduction
                    | InputMode::EditTags => {
                    // Save changes and return to normal mode
                    app.input_mode = InputMode::Normal;
                    app.active_pane = ActivePane::Left;
                    // TODO: Persist changes to backend
                }
                _ => {}
            }
        }
        _ => {}
    }
    Ok(())
}

fn handle_search_mode(app: &mut super::App, key: crossterm::event::KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Esc => {
            app.input_mode = InputMode::Normal;
            app.search_query.clear();
            app.reset_filtered_selection();
        }
        KeyCode::Enter => {
            app.input_mode = InputMode::Normal;
        }
        KeyCode::Tab => {
            app.input_mode = InputMode::Normal;
            // Don't switch panes when exiting search mode
        }
        KeyCode::Backspace => {
            app.search_query.pop();
            app.reset_filtered_selection();
        }
        KeyCode::Char(c) => {
            app.search_query.push(c);
            app.reset_filtered_selection();
        }
        _ => {}
    }
    Ok(())
}

fn handle_inventory_search_mode(app: &mut super::App, key: crossterm::event::KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Esc => {
            app.input_mode = InputMode::Normal;
            app.inventory_search_query.clear();
            app.reset_filtered_selection();
        }
        KeyCode::Enter => {
            app.input_mode = InputMode::Normal;
        }
        KeyCode::Tab => {
            app.input_mode = InputMode::Normal;
            // Don't switch panes when exiting search mode
        }
        KeyCode::Backspace => {
            app.inventory_search_query.pop();
            app.reset_filtered_selection();
        }
        KeyCode::Char(c) => {
            app.inventory_search_query.push(c);
            app.reset_filtered_selection();
        }
        _ => {}
    }
    Ok(())
}

fn handle_create_name_mode(app: &mut super::App, key: crossterm::event::KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Esc => {
            app.input_mode = InputMode::Normal;
            app.create_form.name.clear();
        }
        KeyCode::Down => {
            app.input_mode = InputMode::CreateDescription;
        }
        KeyCode::Up => {
            // Already at first field
        }
        KeyCode::Backspace => {
            app.create_form.name.pop();
        }
        KeyCode::Char(c) => {
            app.create_form.name.push(c);
        }
        KeyCode::Enter => {
            // Save product
            app.save_product()?;
            app.input_mode = InputMode::Normal;
            app.active_pane = ActivePane::Left;
        }
        _ => {}
    }
    Ok(())
}

fn handle_create_description_mode(app: &mut super::App, key: crossterm::event::KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Esc => {
            app.input_mode = InputMode::CreateName;
        }
        KeyCode::Down => {
            app.input_mode = InputMode::CreateCategory;
        }
        KeyCode::Up => {
            app.input_mode = InputMode::CreateName;
        }
        KeyCode::Backspace => {
            app.create_form.description.pop();
        }
        KeyCode::Char(c) => {
            app.create_form.description.push(c);
        }
        KeyCode::Enter => {
            // Save product
            app.save_product()?;
            app.input_mode = InputMode::Normal;
            app.active_pane = ActivePane::Left;
        }
        _ => {}
    }
    Ok(())
}

fn handle_create_category_mode(app: &mut super::App, key: crossterm::event::KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Esc => {
            app.input_mode = InputMode::CreateDescription;
        }
        KeyCode::Tab => {
            app.input_mode = InputMode::CreateCategorySelect;
            app.active_pane = ActivePane::Right;
        }
        KeyCode::Down => {
            app.input_mode = InputMode::CreateProduction;
        }
        KeyCode::Up => {
            app.input_mode = InputMode::CreateDescription;
        }
        KeyCode::Enter => {
            // Save product
            app.save_product()?;
            app.input_mode = InputMode::Normal;
            app.active_pane = ActivePane::Left;
        }
        _ => {}
    }
    Ok(())
}

fn handle_create_category_select_mode(app: &mut super::App, key: crossterm::event::KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Esc => {
            app.input_mode = InputMode::CreateCategory;
            app.active_pane = ActivePane::Left;
        }
        KeyCode::Enter => {
            // Select the current category
            if let Some(category) = app
                .categories
                .get(app.create_form.category_selected_index)
            {
                app.create_form.category_id = category.id;
            }
            app.input_mode = InputMode::CreateCategory;
            app.active_pane = ActivePane::Left;
        }
        KeyCode::Down => {
            if !app.categories.is_empty() {
                app.create_form.category_selected_index =
                    (app.create_form.category_selected_index + 1) % app.categories.len();
            }
        }
        KeyCode::Up => {
            if !app.categories.is_empty() {
                app.create_form.category_selected_index =
                    if app.create_form.category_selected_index == 0 {
                        app.categories.len() - 1
                    } else {
                        app.create_form.category_selected_index - 1
                    };
            }
        }
        KeyCode::Char('n') => {
            // Create new category
            app.item_type = ItemType::Category;
            app.category_form = CategoryForm::default();
            app.popup_field = 0;
            app.previous_input_mode = Some(InputMode::CreateCategorySelect);
            app.input_mode = InputMode::NewCategory;
        }
        _ => {}
    }
    Ok(())
}

fn handle_create_production_mode(app: &mut super::App, key: crossterm::event::KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Esc => {
            app.input_mode = InputMode::CreateCategory;
        }
        KeyCode::Down => {
            app.input_mode = InputMode::CreateTags;
        }
        KeyCode::Up => {
            app.input_mode = InputMode::CreateCategory;
        }
        KeyCode::Left => {
            app.create_form.production = true;
        }
        KeyCode::Right => {
            app.create_form.production = false;
        }
        KeyCode::Char('y') | KeyCode::Char('Y') => {
            app.create_form.production = true;
        }
        KeyCode::Char('n') | KeyCode::Char('N') => {
            app.create_form.production = false;
        }
        KeyCode::Enter => {
            // Save product
            app.save_product()?;
            app.input_mode = InputMode::Normal;
            app.active_pane = ActivePane::Left;
        }
        _ => {}
    }
    Ok(())
}

fn handle_create_tags_mode(app: &mut super::App, key: crossterm::event::KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Esc => {
            app.input_mode = InputMode::CreateProduction;
        }
        KeyCode::Enter => {
            // Save the product
            app.save_product()?;
            app.input_mode = InputMode::Normal;
            app.active_pane = ActivePane::Left;
        }
        KeyCode::Tab => {
            app.tag_selection = vec![false; app.tags.len()];
            // Pre-select tags that are already in create_form.tags
            for (i, tag) in app.tags.iter().enumerate() {
                if app.create_form.tags.contains(tag) {
                    app.tag_selection[i] = true;
                }
            }
            app.tag_select_mode = TagSelectMode::Create;
            app.input_mode = InputMode::CreateTagSelect;
            app.active_pane = ActivePane::Right;
        }
        KeyCode::Up => {
            app.input_mode = InputMode::CreateProduction;
        }
        _ => {}
    }
    Ok(())
}

fn handle_tag_select_mode(app: &mut super::App, key: crossterm::event::KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Esc => {
            app.input_mode = match app.tag_select_mode {
                TagSelectMode::Create => InputMode::CreateTags,
                TagSelectMode::Edit => InputMode::EditTags,
            };
            app.active_pane = ActivePane::Left;
        }
        KeyCode::Enter => {
            // Handle based on context (Create vs Edit)
            match app.tag_select_mode {
                TagSelectMode::Create => {
                    // Add selected tags to create_form.tags
                    app.create_form.tags.clear();
                    for (i, &selected) in app.tag_selection.iter().enumerate() {
                        if selected
                            && let Some(tag) = app.tags.get(i) {
                                app.create_form.tags.push(tag.clone());
                            }
                    }
                    app.tag_selection.clear();
                    app.input_mode = InputMode::CreateTags;
                    app.active_pane = ActivePane::Left;
                }
                TagSelectMode::Edit => {
                    // Update product tags with selected tags
                    if let Some(product) = app.products.get_mut(app.selected_index) {
                        product.tags.clear();
                        for (i, &selected) in app.tag_selection.iter().enumerate() {
                            if selected
                                && let Some(tag) = app.tags.get(i) {
                                    product.tags.push(tag.clone());
                                }
                        }
                        // Update edit_tags_string to reflect changes
                        app.edit_tags_string = product.tags.join(", ");
                    }
                    app.tag_selection.clear();
                    app.input_mode = InputMode::EditTags;
                    app.active_pane = ActivePane::Left;
                }
            }
        }
        KeyCode::Down => {
            if !app.tags.is_empty() {
                app.create_form.tag_selected_index =
                    (app.create_form.tag_selected_index + 1) % app.tags.len();
            }
        }
        KeyCode::Up => {
            if !app.tags.is_empty() {
                app.create_form.tag_selected_index =
                    if app.create_form.tag_selected_index == 0 {
                        app.tags.len() - 1
                    } else {
                        app.create_form.tag_selected_index - 1
                    };
            }
        }
        KeyCode::Char(' ') => {
            // Toggle selection
            if app.create_form.tag_selected_index < app.tag_selection.len() {
                app.tag_selection[app.create_form.tag_selected_index] =
                    !app.tag_selection[app.create_form.tag_selected_index];
            }
        }
        KeyCode::Char('n') => {
            // Create new tag
            app.item_type = ItemType::Tag;
            app.tag_form = TagForm::default();
            app.previous_input_mode = Some(app.input_mode);
            app.input_mode = InputMode::NewTag;
        }
        _ => {}
    }
    Ok(())
}

fn handle_edit_name_mode(app: &mut super::App, key: crossterm::event::KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Esc | KeyCode::Tab => {
            // Cancel changes (discard) and return to normal mode
            if let Some(original_product) = app.edit_backup.take() {
                // Restore original product data
                if let Some(current_product) = app.products.get_mut(app.selected_index) {
                    *current_product = original_product;
                }
            }
            app.input_mode = InputMode::Normal;
            app.active_pane = ActivePane::Left;
        }
        KeyCode::Enter => {
            // Save changes and return to normal mode
            app.edit_backup = None; // Clear backup since we're saving
            if let Some(product) = app.products.get(app.selected_index) {
                let update = crate::api::ProductUpdate {
                    name: Some(product.name.clone()),
                    description: None,
                    tags: None,
                    production: None,
                    material: None,
                    color: None,
                    print_time: None,
                    weight: None,
                    stock_quantity: None,
                    reorder_point: None,
                    unit_cost: None,
                    selling_price: None,
                };
                match app.api_client.update_product(&product.sku, &update) {
                    Ok(_) => {
                        app.status_message = "Product updated successfully".to_string();
                        app.refresh_data();
                    }
                    Err(e) => app.status_message = format!("Error updating product: {:?}", e),
                }
            }
            app.input_mode = InputMode::Normal;
            app.active_pane = ActivePane::Left;
        }
        KeyCode::Down => {
            app.input_mode = InputMode::EditDescription;
        }
        KeyCode::Up => {
            // Already at first field, do nothing
        }
        KeyCode::Backspace => {
            if let Some(product) = app.products.get_mut(app.selected_index) {
                product.name.pop();
            }
        }
        KeyCode::Char(c) => {
            if let Some(product) = app.products.get_mut(app.selected_index) {
                product.name.push(c);
            }
        }
        _ => {}
    }
    Ok(())
}

fn handle_edit_description_mode(app: &mut super::App, key: crossterm::event::KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Esc | KeyCode::Tab => {
            // Cancel changes (discard) and return to normal mode
            if let Some(original_product) = app.edit_backup.take() {
                // Restore original product data
                if let Some(current_product) = app.products.get_mut(app.selected_index) {
                    *current_product = original_product;
                }
            }
            app.input_mode = InputMode::Normal;
            app.active_pane = ActivePane::Left;
        }
        KeyCode::Enter => {
            // Save changes and return to normal mode
            app.edit_backup = None; // Clear backup since we're saving
            if let Some(product) = app.products.get(app.selected_index) {
                let update = crate::api::ProductUpdate {
                    name: None,
                    description: product.description.clone(),
                    tags: None,
                    production: None,
                    material: None,
                    color: None,
                    print_time: None,
                    weight: None,
                    stock_quantity: None,
                    reorder_point: None,
                    unit_cost: None,
                    selling_price: None,
                };
                match app.api_client.update_product(&product.sku, &update) {
                    Ok(_) => {
                        app.status_message = "Product updated successfully".to_string();
                        app.refresh_data();
                    }
                    Err(e) => app.status_message = format!("Error updating product: {:?}", e),
                }
            }
            app.input_mode = InputMode::Normal;
            app.active_pane = ActivePane::Left;
        }
        KeyCode::Down => {
            app.input_mode = InputMode::EditProduction;
        }
        KeyCode::Up => {
            app.input_mode = InputMode::EditName;
        }
        KeyCode::Backspace => {
            if let Some(product) = app.products.get_mut(app.selected_index)
                && let Some(ref mut desc) = product.description
            {
                desc.pop();
            }
        }
        KeyCode::Char(c) => {
            if let Some(product) = app.products.get_mut(app.selected_index) {
                product.description.get_or_insert_with(String::new).push(c);
            }
        }
        _ => {}
    }
    Ok(())
}

fn handle_edit_production_mode(app: &mut super::App, key: crossterm::event::KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Esc | KeyCode::Tab => {
            // Cancel changes (discard) and return to normal mode
            if let Some(original_product) = app.edit_backup.take() {
                // Restore original product data
                if let Some(current_product) = app.products.get_mut(app.selected_index) {
                    *current_product = original_product;
                }
            }
            app.input_mode = InputMode::Normal;
            app.active_pane = ActivePane::Left;
        }
        KeyCode::Enter => {
            // Save changes and return to normal mode
            app.edit_backup = None; // Clear backup since we're saving
            if let Some(product) = app.products.get(app.selected_index) {
                let update = crate::api::ProductUpdate {
                    name: None,
                    description: None,
                    tags: None,
                    production: Some(product.production),
                    material: None,
                    color: None,
                    print_time: None,
                    weight: None,
                    stock_quantity: None,
                    reorder_point: None,
                    unit_cost: None,
                    selling_price: None,
                };
                match app.api_client.update_product(&product.sku, &update) {
                    Ok(_) => {
                        app.status_message = "Product updated successfully".to_string();
                        app.refresh_data();
                    }
                    Err(e) => app.status_message = format!("Error updating product: {:?}", e),
                }
            }
            app.input_mode = InputMode::Normal;
            app.active_pane = ActivePane::Left;
        }
        KeyCode::Up => {
            app.input_mode = InputMode::EditDescription;
        }
        KeyCode::Down => {
            app.input_mode = InputMode::EditTags;
        }
        KeyCode::Left => {
            if let Some(product) = app.products.get_mut(app.selected_index) {
                product.production = true;
            }
        }
        KeyCode::Right => {
            if let Some(product) = app.products.get_mut(app.selected_index) {
                product.production = false;
            }
        }
        KeyCode::Char('y') | KeyCode::Char('Y') => {
            if let Some(product) = app.products.get_mut(app.selected_index) {
                product.production = true;
            }
        }
        KeyCode::Char('n') | KeyCode::Char('N') => {
            if let Some(product) = app.products.get_mut(app.selected_index) {
                product.production = false;
            }
        }
        _ => {}
    }
    Ok(())
}

fn handle_edit_tags_mode(app: &mut super::App, key: crossterm::event::KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Esc => {
            app.input_mode = InputMode::EditProduction;
        }
        KeyCode::Enter => {
            // Parse and save changes
            if let Some(product) = app.products.get_mut(app.selected_index) {
                product.tags = app
                    .edit_tags_string
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
            }
            app.edit_backup = None;
            if let Some(product) = app.products.get(app.selected_index) {
                let update = crate::api::ProductUpdate {
                    name: Some(product.name.clone()),
                    description: product.description.clone(),
                    tags: Some(product.tags.clone()),
                    production: Some(product.production),
                    material: None,
                    color: None,
                    print_time: None,
                    weight: None,
                    stock_quantity: None,
                    reorder_point: None,
                    unit_cost: None,
                    selling_price: None,
                };
                match app.api_client.update_product(&product.sku, &update) {
                    Ok(_) => {
                        app.status_message = "Product updated successfully".to_string();
                        app.refresh_data();
                    }
                    Err(e) => app.status_message = format!("Error updating product: {:?}", e),
                }
            }
            app.input_mode = InputMode::Normal;
            app.active_pane = ActivePane::Left;
        }
        KeyCode::Tab => {
            // Parse current edit_tags_string to product.tags
            if let Some(product) = app.products.get_mut(app.selected_index) {
                product.tags = app
                    .edit_tags_string
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
            }
            app.tag_selection = vec![false; app.tags.len()];
            // Pre-select tags that are already in the current product
            if let Some(product) = app.products.get(app.selected_index) {
                for (i, tag) in app.tags.iter().enumerate() {
                    if product.tags.contains(tag) {
                        app.tag_selection[i] = true;
                    }
                }
            }
            app.tag_select_mode = TagSelectMode::Edit;
            app.input_mode = InputMode::EditTagSelect;
            app.active_pane = ActivePane::Right;
        }
        KeyCode::Up => {
            app.input_mode = InputMode::EditProduction;
        }
        _ => {}
    }
    Ok(())
}

fn handle_new_item_mode(app: &mut super::App, key: crossterm::event::KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Esc => {
            match app.item_type {
                ItemType::Tag => {
                    app.tag_form = TagForm::default();
                    app.input_mode = app.previous_input_mode.unwrap_or(InputMode::CreateTagSelect);
                }
                ItemType::Category => {
                    app.category_form = CategoryForm::default();
                    app.popup_field = 0;
                    app.input_mode = InputMode::CreateCategorySelect;
                }
            }
        }
        KeyCode::Enter => {
            match app.item_type {
                ItemType::Tag => {
                    // Save new tag(s) - support comma-separated
                    if !app.tag_form.name.trim().is_empty() {
                        let tag_names: Vec<String> = app.tag_form.name
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                        
                        let mut created_count = 0;
                        let mut created_tag_names = Vec::new();
                        
                        for tag_name in tag_names {
                            // Skip if tag already exists
                            if app.tags.contains(&tag_name) {
                                continue;
                            }
                            
                            let tag = crate::api::Tag {
                                name: tag_name.clone(),
                                usage_count: 0,
                            };
                            match app.api_client.create_tag(&tag) {
                                Ok(created_tag) => {
                                    app.tags.push(created_tag.name.clone());
                                    created_tag_names.push(created_tag.name.clone());
                                    created_count += 1;
                                }
                                Err(e) => {
                                    app.status_message = format!("Error creating tag '{}': {:?}", tag_name, e);
                                }
                            }
                        }
                        
                        if created_count > 0 {
                            app.tags.sort();
                            app.refresh_data();
                            
                            // Update tag selection array to match new tags length
                            app.tag_selection.resize(app.tags.len(), false);
                            
                            // Pre-select all newly created tags
                            let mut first_new_index = None;
                            for created_tag_name in &created_tag_names {
                                if let Some(index) = app.tags.iter().position(|t| t == created_tag_name) {
                                    app.tag_selection[index] = true;
                                    if first_new_index.is_none() {
                                        first_new_index = Some(index);
                                    }
                                }
                            }
                            
                            // Set selection index to first new tag if available
                            if let Some(first_index) = first_new_index {
                                app.create_form.tag_selected_index = first_index;
                            }
                            
                            let message = if created_count == 1 {
                                format!("Tag '{}' created", created_tag_names.first().unwrap_or(&String::new()))
                            } else {
                                format!("{} tags created", created_count)
                            };
                            app.status_message = message;
                        }
                    } else {
                        app.status_message = "Error: Tag name required".to_string();
                    }
                    app.tag_form = TagForm::default();
                    app.input_mode = match app.tag_select_mode {
                        TagSelectMode::Create => InputMode::CreateTagSelect,
                        TagSelectMode::Edit => InputMode::EditTagSelect,
                    };
                }
                ItemType::Category => {
                    // Save new category
                    if !app.category_form.name.trim().is_empty() && app.category_form.sku.len() == 3 {
                        let category = crate::api::Category {
                            id: None,
                            name: app.category_form.name.clone(),
                            sku_initials: app.category_form.sku.clone(),
                            description: if app.category_form.description.trim().is_empty() {
                                None
                            } else {
                                Some(app.category_form.description.clone())
                            },
                        };
                        match app.api_client.create_category(&category) {
                            Ok(_created_category) => {
                                app.status_message =
                                    format!("Category '{}' created", app.category_form.name);
                                app.refresh_data(); // Refresh to get latest data including new category
                                // Set selection index after refresh
                                app.create_form.category_selected_index = app
                                    .categories
                                    .iter()
                                    .position(|c| c.name == app.category_form.name)
                                    .unwrap_or(0);
                            }
                            Err(e) => {
                                app.status_message = format!("Error creating category: {:?}", e);
                            }
                        }
                    } else {
                        app.status_message = "Error: Name required, SKU must be 3 letters".to_string();
                    }
                    app.category_form = CategoryForm::default();
                    app.popup_field = 0;
                    app.input_mode = InputMode::CreateCategorySelect;
                }
            }
        }
        KeyCode::Backspace => {
            match app.item_type {
                ItemType::Tag => {
                    app.tag_form.name.pop();
                }
                ItemType::Category => match app.popup_field {
                    0 => {
                        app.category_form.name.pop();
                    }
                    1 => {
                        app.category_form.sku.pop();
                    }
                    2 => {
                        app.category_form.description.pop();
                    }
                    _ => {}
                },
            }
        }
        KeyCode::Tab | KeyCode::Down => {
            if app.item_type == ItemType::Category {
                app.popup_field = (app.popup_field + 1) % 3;
            }
        }
        KeyCode::BackTab => {
            if app.item_type == ItemType::Category {
                app.popup_field = if app.popup_field == 0 { 2 } else { app.popup_field - 1 };
            }
        }
        KeyCode::Char(c) => {
            match app.item_type {
                ItemType::Tag => {
                    app.tag_form.name.push(c);
                }
                ItemType::Category => match app.popup_field {
                    0 => {
                        app.category_form.name.push(c);
                    }
                    1 => {
                        if app.category_form.sku.len() < 3 {
                            app.category_form.sku.push(c);
                        }
                    }
                    2 => {
                        app.category_form.description.push(c);
                    }
                    _ => {}
                },
            }
        }
        _ => {}
    }
    Ok(())
}

fn handle_edit_item_mode(app: &mut super::App, key: crossterm::event::KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Esc => {
            match app.edit_item_type {
                ItemType::Tag => {
                    app.tag_form = TagForm::default();
                    app.input_mode = match app.tag_select_mode {
                        TagSelectMode::Create => InputMode::CreateTagSelect,
                        TagSelectMode::Edit => InputMode::EditTagSelect,
                    };
                }
                ItemType::Category => {
                    app.category_form = CategoryForm::default();
                    app.popup_field = 0;
                    app.input_mode = InputMode::CreateCategorySelect;
                }
            }
        }
        KeyCode::Enter => {
            match app.edit_item_type {
                ItemType::Tag => {
                    // Save edited tag
                    if !app.tag_form.name.trim().is_empty() {
                        if app.create_form.tag_selected_index < app.tags.len() {
                            let old_name = app.tags[app.create_form.tag_selected_index].clone();
                            let tag = crate::api::Tag {
                                name: old_name.clone(),
                                usage_count: 0, // Not used for update
                            };
                            let mut updated_tag = tag.clone();
                            updated_tag.name = app.tag_form.name.clone();
                            match app.api_client.update_tag(&updated_tag) {
                                Ok(_) => {
                                    app.tags[app.create_form.tag_selected_index] =
                                        app.tag_form.name.clone();
                                    app.tags.sort();
                                    app.create_form.tag_selected_index = app
                                        .tags
                                        .iter()
                                        .position(|t| t == &app.tag_form.name)
                                        .unwrap_or(app.create_form.tag_selected_index);
                                    app.status_message =
                                        format!("Tag '{}' updated", app.tag_form.name);
                                    app.refresh_data();
                                }
                                Err(e) => {
                                    app.status_message = format!("Error updating tag: {:?}", e);
                                }
                            }
                        }
                    } else {
                        app.status_message = "Error: Tag name required".to_string();
                    }
                    app.tag_form = TagForm::default();
                    app.input_mode = match app.tag_select_mode {
                        TagSelectMode::Create => InputMode::CreateTagSelect,
                        TagSelectMode::Edit => InputMode::EditTagSelect,
                    };
                }
                ItemType::Category => {
                    // Save edited category
                    if !app.category_form.name.trim().is_empty() && app.category_form.sku.len() == 3 {
                        if app.create_form.category_selected_index < app.categories.len() {
                            let mut category =
                                app.categories[app.create_form.category_selected_index].clone();
                            category.name = app.category_form.name.clone();
                            category.sku_initials = app.category_form.sku.clone();
                            category.description = if app.category_form.description.trim().is_empty() {
                                None
                            } else {
                                Some(app.category_form.description.clone())
                            };
                            match app.api_client.update_category(&category) {
                                Ok(updated_category) => {
                                    app.categories[app.create_form.category_selected_index] =
                                        updated_category;
                                    app.categories.sort_by(|a, b| a.name.cmp(&b.name));
                                    app.create_form.category_selected_index = app
                                        .categories
                                        .iter()
                                        .position(|c| c.name == app.category_form.name)
                                        .unwrap_or(app.create_form.category_selected_index);
                                    app.status_message =
                                        format!("Category '{}' updated", app.category_form.name);
                                    app.refresh_data();
                                }
                                Err(e) => {
                                    app.status_message = format!("Error updating category: {:?}", e);
                                }
                            }
                        }
                    } else {
                        app.status_message = "Error: Name required, SKU must be 3 letters".to_string();
                    }
                    app.category_form = CategoryForm::default();
                    app.popup_field = 0;
                    app.input_mode = InputMode::CreateCategorySelect;
                }
            }
        }
        KeyCode::Backspace => {
            app.tag_form.name.pop();
        }
        KeyCode::Char(c) => {
            app.tag_form.name.push(c);
        }
        _ => {}
    }
    Ok(())
}