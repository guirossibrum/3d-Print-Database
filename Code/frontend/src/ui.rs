// ui.rs - Main UI rendering following simplified InputMode

use ratatui::{
    prelude::*,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Tabs, Wrap},
};
use crate::models::{Tab, InputMode};
use crate::state::App;

pub fn draw(f: &mut Frame, app: &mut App, version: &str) {
    let size = f.area();
    
    // Ensure minimum terminal size
    if size.height < 20 || size.width < 80 {
        let error_msg = format!(
            "Terminal too small: {}x{}\nMinimum: 80x20",
            size.width, size.height
        );
        let error = Paragraph::new(error_msg)
            .style(Style::default().fg(Color::Red))
            .block(Block::default().borders(Borders::ALL).title("Error"));
        f.render_widget(error, size);
        return;
    }
    
    // Create layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Length(3), // Tabs
            Constraint::Min(10),  // Content
            Constraint::Length(3), // Footer
        ])
        .split(size);
    
    // Draw header
    draw_header(f, chunks[0]);
    
    // Draw tabs
    draw_tabs(f, chunks[1], app);
    
    // Draw content based on current tab
    draw_content(f, chunks[2], app);
    
    // Draw footer
    draw_footer(f, chunks[3], app, version);
}

fn draw_header(f: &mut Frame, area: Rect) {
    let header = Paragraph::new("3D Print Database TUI")
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(header, area);
}

fn draw_tabs(f: &mut Frame, area: Rect, app: &App) {
    let tab_titles = vec!["Create", "Search", "Inventory"];
    let selected_tab = match app.current_tab {
        Tab::Create => 0,
        Tab::Search => 1,
        Tab::Inventory => 2,
    };
    
    let tabs = Tabs::new(tab_titles.iter().map(|t| Line::from(*t)))
        .select(selected_tab)
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(Style::default().fg(Color::Yellow).bold());
    
    f.render_widget(tabs, area);
}

fn draw_content(f: &mut Frame, area: Rect, app: &App) {
    match app.input_mode {
        InputMode::Normal => match app.current_tab {
            Tab::Search => draw_search_list(f, area, app),
            Tab::Create => draw_create_prompt(f, area, app),
            Tab::Inventory => draw_inventory_summary(f, area, app),
        },
        InputMode::Edit => draw_edit_screen(f, area, app),
        InputMode::Create => draw_create_form(f, area, app),
        InputMode::Select => draw_selection_screen(f, area, app),
        InputMode::Delete => draw_delete_confirmation(f, area, app),
    }
}

fn draw_search_list(f: &mut Frame, area: Rect, app: &App) {
    let content = if app.products.is_empty() {
        "No products found. Make sure backend is running."
    } else {
        "Products loaded. Use arrow keys to navigate, Enter to edit."
    };
    
    let paragraph = Paragraph::new(content)
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL).title("Search"));
    
    f.render_widget(paragraph, area);
}

fn draw_create_prompt(f: &mut Frame, area: Rect, app: &App) {
    let content = "Press 'n' to create new product, Esc to cancel.";
    let paragraph = Paragraph::new(content)
        .style(Style::default().fg(Color::Green))
        .block(Block::default().borders(Borders::ALL).title("Create"));
    
    f.render_widget(paragraph, area);
}

fn draw_inventory_summary(f: &mut Frame, area: Rect, app: &App) {
    let content = format!("Total Products: {}\n\nInventory features coming soon...", app.products.len());
    let paragraph = Paragraph::new(content)
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL).title("Inventory"));
    
    f.render_widget(paragraph, area);
}

fn draw_edit_screen(f: &mut Frame, area: Rect, app: &App) {
    let content = "Edit mode - Use arrow keys to navigate fields, Enter to save, Esc to cancel.";
    let paragraph = Paragraph::new(content)
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Edit Product"));
    
    f.render_widget(paragraph, area);
}

fn draw_create_form(f: &mut Frame, area: Rect, app: &App) {
    let content = "Create form - Fill in product details, Enter to save, Esc to cancel.";
    let paragraph = Paragraph::new(content)
        .style(Style::default().fg(Color::Green))
        .block(Block::default().borders(Borders::ALL).title("Create Product"));
    
    f.render_widget(paragraph, area);
}

fn draw_selection_screen(f: &mut Frame, area: Rect, app: &App) {
    let content = "Selection mode - Use arrow keys to navigate, Space to select, Enter to confirm, Esc to cancel.";
    let paragraph = Paragraph::new(content)
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL).title("Select Items"));
    
    f.render_widget(paragraph, area);
}

fn draw_delete_confirmation(f: &mut Frame, area: Rect, app: &App) {
    let content = "Delete confirmation - Press 'y' to confirm deletion, 'n' to cancel, Esc to go back.";
    let paragraph = Paragraph::new(content)
        .style(Style::default().fg(Color::Red))
        .block(Block::default().borders(Borders::ALL).title("Confirm Delete"));
    
    f.render_widget(paragraph, area);
}

fn draw_footer(f: &mut Frame, area: Rect, app: &App, version: &str) {
    let footer_text = format!(
        "Status: {} | Products: {} | v{}",
        app.status_message,
        app.products.len(),
        version
    );
    
    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL));
    
    f.render_widget(footer, area);
}