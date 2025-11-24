use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Tabs, Wrap},
    Frame,
};

use crate::app::{App, Tab, InputMode};

pub fn draw(f: &mut Frame, app: &mut App) {
    let size = f.size();

    // Clear the frame
    f.render_widget(Clear, size);

    // Ensure minimum terminal size
    if size.height < 20 || size.width < 80 {
        let error_msg = format!("Terminal too small: {}x{}\nMinimum: 80x20", size.width, size.height);
        let error = Paragraph::new(error_msg)
            .style(Style::default().fg(Color::Red))
            .block(Block::default().borders(Borders::ALL).title("Error"));
        f.render_widget(error, size);
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Length(3), // Tabs
            Constraint::Min(10),   // Content
            Constraint::Length(3), // Inventory totals (only for inventory tab)
            Constraint::Length(2), // Footer
        ])
        .split(size);

    // Draw header
    draw_header(f, chunks[0]);

    // Draw tabs
    draw_tabs(f, chunks[1], app);

    // Draw content based on current tab
    let content_area = if matches!(app.current_tab, Tab::Inventory) {
        chunks[2]
    } else {
        Rect {
            x: chunks[2].x,
            y: chunks[2].y,
            width: chunks[2].width,
            height: chunks[2].height + chunks[3].height, // Include totals area for non-inventory
        }
    };

    match app.current_tab {
        Tab::Create => draw_create_tab(f, content_area, app),
        Tab::Search => draw_search_tab(f, content_area, app),
        Tab::Inventory => {
            draw_inventory_tab(f, chunks[2], chunks[3], app);
        }
    }

    // Draw footer
    draw_footer(f, chunks[4], app);
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

    let tabs = Tabs::new(tab_titles)
        .select(selected_tab)
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(Style::default().fg(Color::Yellow).bold());

    f.render_widget(tabs, area);
}

fn draw_create_tab(f: &mut Frame, area: Rect, app: &App) {
    let mut content = vec![
        Line::from(vec![
            Span::styled("Name: ", Style::default().fg(Color::Cyan)),
            Span::raw(&app.create_form.name),
            if matches!(app.input_mode, InputMode::CreateName) {
                Span::styled(" █", Style::default().fg(Color::White))
            } else {
                Span::raw("")
            },
        ]),
        Line::from(vec![
            Span::styled("Description: ", Style::default().fg(Color::Cyan)),
            Span::raw(&app.create_form.description),
            if matches!(app.input_mode, InputMode::CreateDescription) {
                Span::styled(" █", Style::default().fg(Color::White))
            } else {
                Span::raw("")
            },
        ]),
        Line::from(""),
    ];

    // Instructions moved to footer

    let paragraph = Paragraph::new(content)
        .block(Block::default().borders(Borders::ALL).title("Create Product"))
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}

fn draw_search_tab(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    // Left pane: Search input and results
    draw_search_left_pane(f, chunks[0], app);

    // Right pane: Selected product details/edit
    draw_search_right_pane(f, chunks[1], app);
}

fn draw_search_left_pane(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(5)])
        .split(area);

    // Search input
    let search_text = if matches!(app.input_mode, InputMode::Search) {
        format!("Search: {}_", app.search_query)
    } else {
        format!("Search: {} (press '/' to search)", app.search_query)
    };
    let search_paragraph = Paragraph::new(search_text)
        .block(Block::default().borders(Borders::ALL).title("Search Products"));
    f.render_widget(search_paragraph, chunks[0]);

    // Results list
    let items: Vec<ListItem> = app
        .products
        .iter()
        .enumerate()
        .map(|(i, product)| {
            let style = if i == app.selected_index {
                Style::default().fg(Color::Black).bg(Color::Cyan)
            } else {
                Style::default().fg(Color::White)
            };

            ListItem::new(format!("{} - {} ({})",
                product.sku,
                product.name,
                if product.production { "Production" } else { "Prototype" }
            )).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Results"))
        .highlight_style(Style::default().bold());

    f.render_widget(list, chunks[1]);
}

fn draw_search_right_pane(f: &mut Frame, area: Rect, app: &App) {
    if let Some(product) = app.products.get(app.selected_index) {
        let name_style = if matches!(app.input_mode, InputMode::EditName) {
            Style::default().fg(Color::Yellow).bold()
        } else {
            Style::default().fg(Color::Cyan)
        };

        let desc_style = if matches!(app.input_mode, InputMode::EditDescription) {
            Style::default().fg(Color::Yellow).bold()
        } else {
            Style::default().fg(Color::Cyan)
        };

    let mut content = vec![
            Line::from(vec![
                Span::styled("SKU: ", Style::default().fg(Color::Cyan)),
                Span::raw(&product.sku),
            ]),
            Line::from(vec![
                Span::styled("Name: ", name_style),
                Span::raw(&product.name),
                if matches!(app.input_mode, InputMode::EditName) {
                    Span::styled(" █", Style::default().fg(Color::White))
                } else {
                    Span::raw("")
                },
            ]),
            Line::from(vec![
                Span::styled("Description: ", desc_style),
                Span::raw(product.description.as_deref().unwrap_or("")),
                if matches!(app.input_mode, InputMode::EditDescription) {
                    Span::styled(" █", Style::default().fg(Color::White))
                } else {
                    Span::raw("")
                },
            ]),
            Line::from(vec![
                Span::styled("Production: ", Style::default().fg(Color::Cyan)),
                Span::raw(if product.production { "Yes" } else { "No" }),
            ]),
            Line::from(vec![
                Span::styled("Tags: ", Style::default().fg(Color::Cyan)),
                Span::raw(product.tags.join(", ")),
            ]),
            Line::from(""),
        ];

        // Add available tags in a separate section
        if !app.tags.is_empty() {
            content.push(Line::from(vec![Span::styled(
                "Available Tags:",
                Style::default().fg(Color::Green).bold(),
            )]));
            content.push(Line::from(app.tags.join(", ")));
        }

        let paragraph = Paragraph::new(content)
            .block(Block::default().borders(Borders::ALL).title("Product Details"))
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, area);
    } else {
        let paragraph = Paragraph::new("No product selected")
            .block(Block::default().borders(Borders::ALL).title("Product Details"));
        f.render_widget(paragraph, area);
    }
}

fn draw_inventory_tab(f: &mut Frame, content_area: Rect, totals_area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(content_area);

    // Left pane: Product list with inventory columns
    draw_inventory_left_pane(f, chunks[0], app);

    // Right pane: Stock adjustment
    draw_inventory_right_pane(f, chunks[1], app);

    // Bottom pane: Inventory totals
    draw_inventory_totals(f, totals_area, app);
}

fn draw_inventory_left_pane(f: &mut Frame, area: Rect, app: &App) {
    // Header
    let header = vec![
        Line::from("SKU         Name                    Qty   Price   Status")
    ];

    // Product rows
    let mut rows = vec![];
    for (i, product) in app.products.iter().enumerate() {
        let style = if i == app.selected_index {
            Style::default().fg(Color::Black).bg(Color::Cyan)
        } else {
            Style::default().fg(Color::White)
        };

        // Mock inventory data for now (in real app, this would come from API)
        let qty = 10 - (i as i32 * 2); // Mock data
        let price = 25.50 + (i as f64 * 5.0); // Mock data
        let status = if qty > 5 { "In Stock" } else if qty > 0 { "Low Stock" } else { "Out of Stock" };

        rows.push(Line::from(format!("{:<10} {:<20} {:>5} ${:>6.2} {:<10}",
            product.sku,
            &product.name[..product.name.len().min(20)],
            qty,
            price,
            status
        )).style(style));
    }

    let content = [header, rows].concat();

    let paragraph = Paragraph::new(content)
        .block(Block::default().borders(Borders::ALL).title("Inventory"))
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}

fn draw_inventory_right_pane(f: &mut Frame, area: Rect, app: &App) {
    if let Some(product) = app.products.get(app.selected_index) {
    let mut content = vec![
            Line::from(vec![
                Span::styled("Product: ", Style::default().fg(Color::Cyan)),
                Span::raw(&product.name),
            ]),
            Line::from(vec![
                Span::styled("SKU: ", Style::default().fg(Color::Cyan)),
                Span::raw(&product.sku),
            ]),
            Line::from(vec![
                Span::styled("Current Stock: ", Style::default().fg(Color::Cyan)),
                Span::raw("10"), // Mock data
            ]),
            Line::from(vec![
                Span::styled("Reorder Point: ", Style::default().fg(Color::Cyan)),
                Span::raw("5"), // Mock data
            ]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Press '+' to increase, '-' to decrease stock",
                Style::default().fg(Color::Green),
            )]),
            Line::from(vec![Span::styled(
                "Press 'r' to set reorder point",
                Style::default().fg(Color::Green),
            )]),
        ];

        let paragraph = Paragraph::new(content)
            .block(Block::default().borders(Borders::ALL).title("Stock Adjustment"))
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, area);
    } else {
        let paragraph = Paragraph::new("No product selected")
            .block(Block::default().borders(Borders::ALL).title("Stock Adjustment"));
        f.render_widget(paragraph, area);
    }
}

fn draw_inventory_totals(f: &mut Frame, area: Rect, app: &App) {
    // Mock totals - in real app, this would be calculated from API data
    let total_products = app.products.len();
    let total_value = 1250.75; // Mock data
    let low_stock_items = 3; // Mock data

    let content = format!("Total Products: {} | Total Value: ${:.2} | Low Stock Items: {}",
        total_products, total_value, low_stock_items);

    let paragraph = Paragraph::new(content)
        .block(Block::default().borders(Borders::ALL).title("Inventory Summary"))
        .style(Style::default().fg(Color::Green));

    f.render_widget(paragraph, area);
}

fn draw_footer(f: &mut Frame, area: Rect, app: &App) {
    // Get instructions based on current tab and mode
    let instructions = match app.current_tab {
        Tab::Create => match app.input_mode {
            InputMode::Normal => "n: create, Tab: switch tabs",
            InputMode::CreateName => "Enter name, Enter: next, Esc: cancel",
            InputMode::CreateDescription => "Enter desc, Enter: save, Esc: cancel",
            _ => "Tab: switch tabs",
        },
        Tab::Search => match app.input_mode {
            InputMode::Normal => "Tab: edit, j/k: select, d: delete",
            InputMode::EditName => "→: desc, ←: back, ↑: cancel, Enter: desc",
            InputMode::EditDescription => "←: name, ↑: cancel, Enter: save",
            _ => "Tab: edit, j/k: select",
        },
        Tab::Inventory => "j/k: navigate, +/-/Enter: adjust stock",
    };

    // Truncate status message if too long
    let max_status_len = 30;
    let truncated_status = if app.status_message.len() > max_status_len {
        format!("{}...", &app.status_message[..max_status_len.saturating_sub(3)])
    } else {
        app.status_message.clone()
    };

    let footer_text = format!("{} | {} | q:quit v0.3.0", truncated_status, instructions);

    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(Color::White).bg(Color::Blue).bold())
        .block(Block::default().borders(Borders::ALL))
        .wrap(Wrap { trim: false });

    f.render_widget(footer, area);
}