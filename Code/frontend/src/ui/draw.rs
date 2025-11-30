// ui/draw.rs - Main UI dispatcher and mode-based rendering system

use ratatui::{
    prelude::*,
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
};
use crate::models::{Tab, InputMode};
use crate::state::App;

// Style constants for consistent UI appearance
const HEADER_STYLE: Style = Style::new().fg(Color::Green).add_modifier(Modifier::BOLD);
const ACTIVE_TAB_STYLE: Style = Style::new().fg(Color::Yellow).add_modifier(Modifier::BOLD);
const INACTIVE_TAB_STYLE: Style = Style::new().fg(Color::Gray);
const SELECTED_ITEM_STYLE: Style = Style::new().fg(Color::Black).bg(Color::White);
const NORMAL_STYLE: Style = Style::new().fg(Color::White);
const ERROR_STYLE: Style = Style::new().fg(Color::Red);
const SUCCESS_STYLE: Style = Style::new().fg(Color::Green);
const FOOTER_STYLE: Style = Style::new().fg(Color::Cyan);

/// Main draw dispatcher - routes to mode-specific rendering functions
pub fn draw(f: &mut Frame, app: &App) {
    let size = f.area();

    // Ensure minimum terminal size
    if size.height < 20 || size.width < 80 {
        draw_size_error(f, size);
        return;
    }

    // Create main layout
    let chunks = create_main_layout(size);

    // Draw static elements
    draw_header(f, chunks[0]);
    draw_tabs(f, chunks[1], app.current_tab());

    // Mode-based content rendering
    match app.input_mode() {
        InputMode::Normal => draw_normal_mode(f, chunks[2], app),
        InputMode::Edit => draw_edit_mode(f, chunks[2], app),
        InputMode::Create => draw_create_mode(f, chunks[2], app),
        InputMode::Select => draw_select_mode(f, chunks[2], app),
        InputMode::Delete => draw_delete_mode(f, chunks[2], app),
        // Edit sub-modes
        InputMode::EditName | InputMode::EditDescription | InputMode::EditCategory |
        InputMode::EditProduction | InputMode::EditTags | InputMode::EditMaterials => {
            draw_edit_mode(f, chunks[2], app)
        }
        // Delete sub-modes
        InputMode::DeleteConfirm | InputMode::DeleteFileConfirm => {
            draw_delete_mode(f, chunks[2], app)
        }
    }

    // Draw popups if active
    if should_show_popup(app) {
        draw_popup_overlay(f, chunks[2], app);
    }

    // Draw footer (only when no popup is active)
    if !should_show_popup(app) {
        draw_footer(f, chunks[3], app);
    }
}

/// Create the main vertical layout for the application
fn create_main_layout(area: Rect) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Length(3), // Tabs
            Constraint::Min(10),   // Content
            Constraint::Length(3), // Footer
        ])
        .split(area).to_vec()
}

/// Create horizontal content panes (left/right split)
fn create_content_panes(area: Rect) -> (Rect, Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);
    
    (chunks[0], chunks[1])
}

/// Draw error message when terminal is too small
fn draw_size_error(f: &mut Frame, area: Rect) {
    let error_msg = format!(
        "Terminal too small: {}x{}\nMinimum: 80x20",
        area.width, area.height
    );
    let error = Paragraph::new(error_msg)
        .style(ERROR_STYLE)
        .block(Block::default().borders(Borders::ALL).title("Error"));
    f.render_widget(error, area);
}

/// Draw application header
fn draw_header(f: &mut Frame, area: Rect) {
    let header = Paragraph::new("3D Print Database TUI")
        .style(HEADER_STYLE)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(header, area);
}

/// Draw tab navigation
fn draw_tabs(f: &mut Frame, area: Rect, current_tab: Tab) {
    let tabs = vec!["Create", "Search", "Inventory"];
    let selected_index = match current_tab {
        Tab::Create => 0,
        Tab::Search => 1,
        Tab::Inventory => 2,
    };

    let tabs_widget = ratatui::widgets::Tabs::new(tabs)
        .select(selected_index)
        .style(INACTIVE_TAB_STYLE)
        .highlight_style(ACTIVE_TAB_STYLE)
        .divider(" | ")
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(tabs_widget, area);
}

/// Draw normal mode content based on current tab
fn draw_normal_mode(f: &mut Frame, area: Rect, app: &App) {
    match app.current_tab() {
        Tab::Search => draw_search_tab(f, area, app),
        Tab::Create => draw_create_tab(f, area, app),
        Tab::Inventory => draw_inventory_tab(f, area, app),
    }
}

/// Draw edit mode content based on current tab
fn draw_edit_mode(f: &mut Frame, area: Rect, app: &App) {
    match app.current_tab() {
        Tab::Search => draw_edit_search_tab(f, area, app),
        Tab::Create => draw_edit_create_tab(f, area, app),
        Tab::Inventory => draw_edit_inventory_tab(f, area, app),
    }
}

/// Draw create mode content based on current tab
fn draw_create_mode(f: &mut Frame, area: Rect, app: &App) {
    match app.current_tab() {
        Tab::Search => draw_create_search_tab(f, area, app),
        Tab::Create => draw_create_create_tab(f, area, app),
        Tab::Inventory => draw_create_inventory_tab(f, area, app),
    }
}

/// Draw select mode content (popup overlay)
fn draw_select_mode(f: &mut Frame, area: Rect, app: &App) {
    draw_selection_popup(f, area, app);
}

/// Draw delete mode content (popup overlay)
fn draw_delete_mode(f: &mut Frame, area: Rect, app: &App) {
    draw_delete_popup(f, area, app);
}

/// Draw search tab in normal mode (product list + details)
fn draw_search_tab(f: &mut Frame, area: Rect, app: &App) {
    let (list_area, details_area) = create_content_panes(area);
    
    // Left: Product list
    draw_product_list(f, list_area, app.products(), app.selected_index());
    
    // Right: Product details
    if let Some(product) = app.selected_product() {
        draw_product_details(f, details_area, product);
    } else {
        draw_empty_details(f, details_area, "No product selected");
    }
}

/// Draw create tab in normal mode (instructions + options)
fn draw_create_tab(f: &mut Frame, area: Rect, app: &App) {
    let (instructions_area, options_area) = create_content_panes(area);
    
    // Left: Creation instructions
    draw_create_instructions(f, instructions_area);
    
    // Right: Available options
    draw_available_options(f, options_area, app);
}

/// Draw inventory tab in normal mode (inventory table + summary)
fn draw_inventory_tab(f: &mut Frame, area: Rect, app: &App) {
    let (table_area, summary_area) = create_content_panes(area);
    
    // Left: Inventory table
    draw_inventory_table(f, table_area, app.products());
    
    // Right: Summary statistics
    draw_inventory_summary(f, summary_area, app.products());
}

/// Draw search tab in edit mode (edit form + preview)
fn draw_edit_search_tab(f: &mut Frame, area: Rect, app: &App) {
    let (form_area, preview_area) = create_content_panes(area);
    
    // Left: Edit form
    draw_edit_form(f, form_area, app);
    
    // Right: Live preview
    draw_product_preview(f, preview_area, app);
}

/// Draw create tab in edit mode (create or edit product)
fn draw_edit_create_tab(f: &mut Frame, area: Rect, app: &App) {
    let (form_area, options_area) = create_content_panes(area);

    // Left: Unified product form
    draw_product_form(f, form_area, app);

    // Right: Available tags/materials/categories
    draw_creation_options(f, options_area, app);
}

/// Draw inventory tab in edit mode (edit inventory)
fn draw_edit_inventory_tab(f: &mut Frame, area: Rect, _app: &App) {
    let (edit_area, preview_area) = create_content_panes(area);
    
    // Left: Inventory editing interface
    draw_inventory_edit(f, edit_area, _app);
    
    // Right: Preview of changes
    draw_inventory_preview(f, preview_area, _app);
}

/// Draw search tab in create mode (create product form)
fn draw_create_search_tab(f: &mut Frame, area: Rect, app: &App) {
    let (form_area, options_area) = create_content_panes(area);

    // Left: Unified product form
    draw_product_form(f, form_area, app);

    // Right: Available tags/materials/categories
    draw_creation_options(f, options_area, app);
}

/// Draw create tab in create mode (create new product)
fn draw_create_create_tab(f: &mut Frame, area: Rect, app: &App) {
    let (form_area, options_area) = create_content_panes(area);

    // Left: Unified product form
    draw_product_form(f, form_area, app);

    // Right: Available tags/materials/categories
    draw_creation_options(f, options_area, app);
}

/// Draw inventory tab in create mode (add inventory items)
fn draw_create_inventory_tab(f: &mut Frame, area: Rect, _app: &App) {
    draw_placeholder(f, area, "Create inventory mode - to be implemented");
}

/// Draw product list widget
fn draw_product_list(f: &mut Frame, area: Rect, products: &[crate::models::Product], selected_index: usize) {
    let items: Vec<Line> = products
        .iter()
        .enumerate()
        .map(|(i, product)| {
            let style = if i == selected_index {
                SELECTED_ITEM_STYLE
            } else {
                NORMAL_STYLE
            };
            Line::from(format!("{} - {}", product.sku, product.name)).style(style)
        })
        .collect();

    let list = ratatui::widgets::List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Products"))
        .style(NORMAL_STYLE);

    f.render_widget(list, area);
}

/// Draw product details widget
fn draw_product_details(f: &mut Frame, area: Rect, product: &crate::models::Product) {
    let details = vec![
        Line::from(format!("Name: {}", product.name)),
        Line::from(format!("SKU: {}", product.sku)),
        Line::from(format!("Description: {}", product.description.as_deref().unwrap_or("None"))),
        Line::from(format!("Production: {}", if product.production { "Yes" } else { "No" })),
        Line::from(format!("Category: {}", product.category.as_ref().map(|c| &c.name).map_or("None", |v| v.as_str()))),
        Line::from(format!("Tags: {}", product.tags.iter().map(|t| t.name.as_str()).collect::<Vec<_>>().join(", "))),
        Line::from(format!("Materials: {}", product.materials.iter().map(|m| m.name.as_str()).collect::<Vec<_>>().join(", "))),
    ];

    let details_widget = Paragraph::new(details)
        .block(Block::default().borders(Borders::ALL).title("Details"))
        .style(NORMAL_STYLE)
        .wrap(Wrap { trim: true });

    f.render_widget(details_widget, area);
}

/// Draw empty details placeholder
fn draw_empty_details(f: &mut Frame, area: Rect, message: &str) {
    let placeholder = Paragraph::new(message)
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL).title("Details"));
    f.render_widget(placeholder, area);
}

/// Draw creation instructions
fn draw_create_instructions(f: &mut Frame, area: Rect) {
    let instructions = vec![
        Line::from("press [n] to create new product"),
    ];

    let instructions_widget = Paragraph::new(instructions)
        .block(Block::default().borders(Borders::ALL).title("Create"))
        .style(NORMAL_STYLE)
        .wrap(Wrap { trim: true });

    f.render_widget(instructions_widget, area);
}

/// Draw available options for creation
fn draw_available_options(f: &mut Frame, area: Rect, _app: &App) {
    let options_widget = Paragraph::new("")
        .block(Block::default().borders(Borders::ALL).title(""))
        .style(NORMAL_STYLE);

    f.render_widget(options_widget, area);
}

/// Draw inventory table
fn draw_inventory_table(f: &mut Frame, area: Rect, products: &[crate::models::Product]) {
    let rows: Vec<Line> = products
        .iter()
        .map(|product| {
            Line::from(format!(
                "{} | {} | {} | {}",
                product.sku,
                product.name,
                product.stock_quantity.unwrap_or(0),
                if product.production { "Yes" } else { "No" }
            ))
        })
        .collect();

    let table = Paragraph::new(rows)
        .block(Block::default().borders(Borders::ALL).title("Inventory"))
        .style(NORMAL_STYLE);

    f.render_widget(table, area);
}

/// Draw inventory summary
fn draw_inventory_summary(f: &mut Frame, area: Rect, products: &[crate::models::Product]) {
    let total_products = products.len();
    let in_production = products.iter().filter(|p| p.production).count();
    let total_stock: i32 = products.iter().map(|p| p.stock_quantity.unwrap_or(0)).sum();

    let summary = vec![
        Line::from("Inventory Summary:"),
        Line::from(""),
        Line::from(format!("Total Products: {}", total_products)),
        Line::from(format!("In Production: {}", in_production)),
        Line::from(format!("Total Stock: {}", total_stock)),
    ];

    let summary_widget = Paragraph::new(summary)
        .block(Block::default().borders(Borders::ALL).title("Summary"))
        .style(NORMAL_STYLE);

    f.render_widget(summary_widget, area);
}

/// Draw edit form for products
fn draw_edit_form(f: &mut Frame, area: Rect, _app: &App) {
    let form_text = vec![
        Line::from("Edit Product Form:"),
        Line::from(""),
        Line::from("Name: [editable field]"),
        Line::from("Description: [editable field]"),
        Line::from("Category: [selectable field]"),
        Line::from("Production: [toggle field]"),
        Line::from("Tags: [multi-select field]"),
        Line::from("Materials: [multi-select field]"),
    ];

    let form_widget = Paragraph::new(form_text)
        .block(Block::default().borders(Borders::ALL).title("Edit Form"))
        .style(NORMAL_STYLE)
        .wrap(Wrap { trim: true });

    f.render_widget(form_widget, area);
}

/// Draw product preview
fn draw_product_preview(f: &mut Frame, area: Rect, app: &App) {
    let preview = vec![
        Line::from("Live Preview:"),
        Line::from(""),
        Line::from(format!("Name: {}", app.selected_product().map(|p| p.name.as_str()).unwrap_or("None"))),
        Line::from(format!("SKU: {}", app.selected_product().map(|p| p.sku.as_str()).unwrap_or("None"))),
        Line::from(format!("Production: {}", app.selected_product().map(|p| if p.production { "Yes" } else { "No" }).unwrap_or("None"))),
    ];

    let preview_widget = Paragraph::new(preview)
        .block(Block::default().borders(Borders::ALL).title("Preview"))
        .style(NORMAL_STYLE)
        .wrap(Wrap { trim: true });

    f.render_widget(preview_widget, area);
}

/// Draw inventory editing interface
fn draw_inventory_edit(f: &mut Frame, area: Rect, _app: &App) {
    let edit_text = vec![
        Line::from("Inventory Editing:"),
        Line::from(""),
        Line::from("Stock Quantity: [editable]"),
        Line::from("Reorder Point: [editable]"),
        Line::from("Unit Cost: [editable]"),
        Line::from("Selling Price: [editable]"),
    ];

    let edit_widget = Paragraph::new(edit_text)
        .block(Block::default().borders(Borders::ALL).title("Edit Inventory"))
        .style(NORMAL_STYLE)
        .wrap(Wrap { trim: true });

    f.render_widget(edit_widget, area);
}

/// Draw inventory preview
fn draw_inventory_preview(f: &mut Frame, area: Rect, _app: &App) {
    let preview = vec![
        Line::from("Inventory Preview:"),
        Line::from(""),
        Line::from("Updated values will appear here"),
        Line::from("as you make changes."),
    ];

    let preview_widget = Paragraph::new(preview)
        .block(Block::default().borders(Borders::ALL).title("Preview"))
        .style(NORMAL_STYLE)
        .wrap(Wrap { trim: true });

    f.render_widget(preview_widget, area);
}

/// Draw unified product form (create or edit)
fn draw_product_form(f: &mut Frame, area: Rect, app: &App) {
    use crate::models::InputMode;

    let product = match app.get_current_product() {
        Some(p) => p,
        None => return,
    };

    let title = if app.is_create_mode() { "Create Product" } else { "Edit Product" };

    // Map input mode to field index
    let current_field = match app.input_mode() {
        InputMode::EditName => 0,
        InputMode::EditDescription => 1,
        InputMode::EditCategory => 2,
        InputMode::EditProduction => 3,
        InputMode::EditTags => 4,
        InputMode::EditMaterials => 5,
        _ => 0,
    };

    let mut form_lines = vec![
        Line::from(format!("{}:", if app.is_create_mode() { "Create New Product" } else { "Edit Product" })),
        Line::from(""),
    ];

    let fields = vec![
        ("Name", format!("{}", product.name)),
        ("Description", product.description.as_deref().unwrap_or("").to_string()),
        ("Category", product.category.as_ref().map(|c| c.name.as_str()).unwrap_or("None").to_string()),
        ("Production", if product.production { "Yes" } else { "No" }.to_string()),
        ("Tags", product.tags.iter().map(|t| t.name.as_str()).collect::<Vec<_>>().join(", ")),
        ("Materials", product.materials.iter().map(|m| m.name.as_str()).collect::<Vec<_>>().join(", ")),
    ];

    for (i, (label, value)) in fields.iter().enumerate() {
        let style = if i == current_field { SELECTED_ITEM_STYLE } else { NORMAL_STYLE };
        form_lines.push(Line::from(format!("{}: {}", label, value)).style(style));
    }

    let form_widget = Paragraph::new(form_lines)
        .block(Block::default().borders(Borders::ALL).title(title))
        .style(NORMAL_STYLE)
        .wrap(Wrap { trim: true });

    f.render_widget(form_widget, area);
}

/// Draw creation options
fn draw_creation_options(f: &mut Frame, area: Rect, app: &App) {
    let mut options = vec![
        Line::from("Creation Options:"),
        Line::from(""),
        Line::from("Available Categories:"),
    ];

    for category in app.categories() {
        options.push(Line::from(format!("• {}", category.name)));
    }

    options.push(Line::from(""));
    options.push(Line::from("Available Tags:"));

    for tag in app.tags() {
        options.push(Line::from(format!("• {}", tag.name)));
    }

    options.push(Line::from(""));
    options.push(Line::from("Available Materials:"));

    for material in app.materials() {
        options.push(Line::from(format!("• {}", material.name)));
    }

    let options_widget = Paragraph::new(options)
        .block(Block::default().borders(Borders::ALL).title("Options"))
        .style(NORMAL_STYLE)
        .wrap(Wrap { trim: true });

    f.render_widget(options_widget, area);
}

/// Draw item creation form (for tags, materials, categories)
fn draw_item_creation_form(f: &mut Frame, area: Rect, _app: &App) {
    let form_text = vec![
        Line::from("Create New Item:"),
        Line::from(""),
        Line::from("Select type to create:"),
        Line::from("1. Category"),
        Line::from("2. Tag"),
        Line::from("3. Material"),
        Line::from(""),
        Line::from("Name: [input field]"),
        Line::from("Description: [optional]"),
    ];

    let form_widget = Paragraph::new(form_text)
        .block(Block::default().borders(Borders::ALL).title("Create Item"))
        .style(NORMAL_STYLE)
        .wrap(Wrap { trim: true });

    f.render_widget(form_widget, area);
}

/// Draw existing items list
fn draw_existing_items_list(f: &mut Frame, area: Rect, _app: &App) {
    let items = vec![
        Line::from("Existing Items:"),
        Line::from(""),
        Line::from("Categories:"),
        Line::from("• Category 1"),
        Line::from("• Category 2"),
        Line::from(""),
        Line::from("Tags:"),
        Line::from("• Tag 1"),
        Line::from("• Tag 2"),
        Line::from(""),
        Line::from("Materials:"),
        Line::from("• Material 1"),
        Line::from("• Material 2"),
    ];

    let list_widget = Paragraph::new(items)
        .block(Block::default().borders(Borders::ALL).title("Existing Items"))
        .style(NORMAL_STYLE)
        .wrap(Wrap { trim: true });

    f.render_widget(list_widget, area);
}

/// Draw placeholder for unimplemented features
fn draw_placeholder(f: &mut Frame, area: Rect, message: &str) {
    let placeholder = Paragraph::new(message)
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL).title("Not Implemented"));
    f.render_widget(placeholder, area);
}

/// Check if popup should be shown
fn should_show_popup(app: &App) -> bool {
    matches!(app.input_mode(), InputMode::Select | InputMode::Delete)
}

/// Draw popup overlay (router for different popup types)
fn draw_popup_overlay(f: &mut Frame, area: Rect, app: &App) {
    match app.input_mode() {
        InputMode::Select => draw_selection_popup(f, area, app),
        InputMode::Delete => draw_delete_popup(f, area, app),
        // Edit sub-modes
        InputMode::EditName | InputMode::EditDescription | InputMode::EditCategory |
        InputMode::EditProduction | InputMode::EditTags | InputMode::EditMaterials => {
            // Edit modes don't show popups in current implementation
        }
        // Delete sub-modes
        InputMode::DeleteConfirm | InputMode::DeleteFileConfirm => {
            draw_delete_popup(f, area, app)
        }
        _ => {}
    }
}

/// Draw popup overlay for selection mode
fn draw_selection_popup(f: &mut Frame, area: Rect, _app: &App) {
    let popup_area = centered_rect(60, 70, area);
    
    f.render_widget(Clear, popup_area);
    
    let popup = Block::default()
        .title("Select Items")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Yellow));
    
    f.render_widget(popup, popup_area);
    
    let list_area = popup_area.inner(ratatui::layout::Margin::new(1, 1));
    let selection_text = vec![
        Line::from("Use UP/DOWN to navigate"),
        Line::from("Use SPACE to select/deselect"),
        Line::from("Use ENTER to confirm"),
        Line::from("Use ESC to cancel"),
    ];
    
    let list = Paragraph::new(selection_text)
        .style(NORMAL_STYLE)
        .wrap(Wrap { trim: true });
    
    f.render_widget(list, list_area);
}

/// Draw popup overlay for delete confirmation
fn draw_delete_popup(f: &mut Frame, area: Rect, _app: &App) {
    let popup_area = centered_rect(40, 20, area);
    
    f.render_widget(Clear, popup_area);
    
    let confirm_text = vec![
        Line::from("Delete this item?"),
        Line::from(""),
        Line::from("y: Yes  n: No  ESC: Cancel"),
    ];
    
    let popup = Paragraph::new(confirm_text)
        .block(Block::default()
            .title("Confirm Delete")
            .borders(Borders::ALL))
        .style(ERROR_STYLE)
        .wrap(Wrap { trim: true });
    
    f.render_widget(popup, popup_area);
}

/// Draw footer with status and instructions
fn draw_footer(f: &mut Frame, area: Rect, app: &App) {
    let left_text = format!(
        "Status: {} | Tab: {:?} | Mode: {:?}",
        app.status_message(),
        app.current_tab(),
        app.input_mode()
    );

    let right_text = format!("v{}", env!("CARGO_PKG_VERSION"));

    let footer_block = Block::default().borders(Borders::ALL);

    let inner_area = footer_block.inner(area);
    f.render_widget(footer_block, area);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(10), Constraint::Length(right_text.len() as u16 + 2)])
        .split(inner_area);

    let left = Paragraph::new(left_text)
        .style(FOOTER_STYLE);
    f.render_widget(left, chunks[0]);

    let right = Paragraph::new(right_text)
        .style(FOOTER_STYLE)
        .alignment(Alignment::Right);
    f.render_widget(right, chunks[1]);
}

/// Helper function to create centered rectangle for popups
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}