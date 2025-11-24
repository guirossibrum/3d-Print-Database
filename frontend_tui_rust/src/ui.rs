use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Tabs, Wrap},
    Frame,
};

use crate::app::{App, Tab, InputMode, ActivePane};

pub fn draw(f: &mut Frame, app: &mut App, version: &str) {
    let size = f.area();

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
            Constraint::Length(3), // Footer
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
    draw_footer(f, chunks[4], app, version);
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
    let content = vec![
        Line::from(vec![
            Span::styled("Name: ", Style::default().fg(Color::Cyan)),
            Span::raw(&app.create_form.name),
            if matches!(app.input_mode, InputMode::CreateName) {
                Span::styled(" _", Style::default().fg(Color::White))
            } else {
                Span::raw("")
            },
        ]),
        Line::from(vec![
            Span::styled("Description: ", Style::default().fg(Color::Cyan)),
            Span::raw(&app.create_form.description),
            if matches!(app.input_mode, InputMode::CreateDescription) {
                Span::styled(" _", Style::default().fg(Color::White))
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

/// Generic searchable pane component that can be reused across different tabs
fn draw_searchable_pane_with_styles<F>(
    f: &mut Frame,
    area: Rect,
    app: &App,
    title: &str,
    search_query: &str,
    _input_mode: InputMode,
    search_border_style: Style,
    results_border_style: Style,
    display_callback: F,
) where
    F: Fn(&mut Frame, Rect, &App, &[&crate::api::Product], Style),
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(5)])
        .split(area);

    // Search input
    let search_text = if matches!(app.input_mode, _input_mode) {
        format!("{}_", search_query)
    } else {
        "_".to_string()
    };
    let search_paragraph = Paragraph::new(search_text)
        .block(Block::default().borders(Borders::ALL).title(title).border_style(search_border_style));
    f.render_widget(search_paragraph, chunks[0]);

    // Filter products based on search query
    let filtered_products: Vec<&crate::api::Product> = if search_query.is_empty() {
        app.products.iter().collect()
    } else {
        app.products.iter()
            .filter(|product|
                product.name.to_lowercase().contains(&search_query.to_lowercase()) ||
                product.sku.to_lowercase().contains(&search_query.to_lowercase())
            )
            .collect()
    };

    // Display results
    display_callback(f, chunks[1], app, &filtered_products, results_border_style);
}

/// Display callback for simple list format (used by Search tab)
fn display_as_list(f: &mut Frame, area: Rect, app: &App, products: &[&crate::api::Product], border_style: Style) {
    let mut content_lines = vec![];

    // Add search instruction if not in search mode
    if !matches!(app.input_mode, InputMode::Search) {
        content_lines.push(Line::from(vec![
            Span::styled("Press ", Style::default().fg(Color::Gray)),
            Span::styled("/", Style::default().fg(Color::Cyan).bold()),
            Span::styled(" to search", Style::default().fg(Color::Gray)),
        ]));
        content_lines.push(Line::from(""));
    }

    // Add the list items
    for (i, product) in products.iter().enumerate() {
        let style = if i == app.selected_index {
            Style::default().fg(Color::Black).bg(Color::Cyan)
        } else {
            Style::default().fg(Color::White)
        };

        content_lines.push(Line::from(Span::styled(
            format!("{} - {} ({})",
                product.sku,
                product.name,
                if product.production { "Production" } else { "Prototype" }
            ),
            style
        )));
    }

    let paragraph = Paragraph::new(content_lines)
        .block(Block::default().borders(Borders::ALL).title("Results").border_style(border_style))
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}

/// Display callback for table format (used by Inventory tab)
fn display_as_table(f: &mut Frame, area: Rect, app: &App, products: &[&crate::api::Product], border_style: Style) {
    // Header
    let header = vec![
        Line::from("SKU         Name                    Qty   Price   Status")
    ];

    // Product rows
    let mut rows = vec![];
    for (i, product) in products.iter().enumerate() {
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
        .block(Block::default().borders(Borders::ALL).title("Inventory Results").border_style(border_style))
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}

fn draw_search_left_pane(f: &mut Frame, area: Rect, app: &App) {
    let search_border_style = if matches!(app.input_mode, InputMode::Search) {
        Style::default().fg(Color::Yellow).bold()
    } else {
        Style::default().fg(Color::White)
    };

    let results_border_style = if matches!(app.active_pane, ActivePane::Left) && !matches!(app.input_mode, InputMode::Search) {
        Style::default().fg(Color::Yellow).bold()
    } else {
        Style::default().fg(Color::White)
    };

    let title = "Search";

    draw_searchable_pane_with_styles(
        f,
        area,
        app,
        title,
        &app.search_query,
        InputMode::Search,
        search_border_style,
        results_border_style,
        display_as_list,
    );
}

fn draw_search_right_pane(f: &mut Frame, area: Rect, app: &App) {
    let border_style = if matches!(app.active_pane, ActivePane::Right) {
        Style::default().fg(Color::Yellow).bold()
    } else {
        Style::default().fg(Color::White)
    };

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

        let prod_style = if matches!(app.input_mode, InputMode::EditProduction) {
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
                    Span::styled("_", Style::default().fg(Color::White))
                } else {
                    Span::raw("")
                },
            ]),
            Line::from(vec![
                Span::styled("Description: ", desc_style),
                Span::raw(product.description.as_deref().unwrap_or("")),
                if matches!(app.input_mode, InputMode::EditDescription) {
                    Span::styled("_", Style::default().fg(Color::White))
                } else {
                    Span::raw("")
                },
            ]),
            Line::from(vec![
                Span::styled("Production: ", prod_style),
                Span::raw(if matches!(app.input_mode, InputMode::EditProduction) {
                    format!("[{}] Yes    [{}] No",
                        if product.production { "x" } else { " " },
                        if !product.production { "x" } else { " " })
                } else {
                    (if product.production { "Yes" } else { "No" }).to_string()
                }),
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
            .block(Block::default().borders(Borders::ALL).title("Product Details").border_style(border_style))
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, area);
    } else {
        let paragraph = Paragraph::new("No product selected")
            .block(Block::default().borders(Borders::ALL).title("Product Details").border_style(border_style));
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
    let search_border_style = if matches!(app.input_mode, InputMode::InventorySearch) {
        Style::default().fg(Color::Yellow).bold()
    } else {
        Style::default().fg(Color::White)
    };

    let results_border_style = if matches!(app.active_pane, ActivePane::Left) && !matches!(app.input_mode, InputMode::InventorySearch) {
        Style::default().fg(Color::Yellow).bold()
    } else {
        Style::default().fg(Color::White)
    };

    let title = "Search";

    draw_searchable_pane_with_styles(
        f,
        area,
        app,
        title,
        &app.inventory_search_query,
        InputMode::InventorySearch,
        search_border_style,
        results_border_style,
        display_as_table,
    );
}

fn draw_inventory_right_pane(f: &mut Frame, area: Rect, app: &App) {
    let border_style = if matches!(app.active_pane, ActivePane::Right) {
        Style::default().fg(Color::Yellow).bold()
    } else {
        Style::default().fg(Color::White)
    };

    if let Some(product) = app.products.get(app.selected_index) {
    let content = vec![
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
            .block(Block::default().borders(Borders::ALL).title("Stock Adjustment").border_style(border_style))
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, area);
    } else {
        let paragraph = Paragraph::new("No product selected")
            .block(Block::default().borders(Borders::ALL).title("Stock Adjustment").border_style(border_style));
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

fn draw_footer(f: &mut Frame, area: Rect, app: &App, version: &str) {
    // Get instructions based on current tab, pane, and mode
    let instructions = match app.current_tab {
        Tab::Create => match app.input_mode {
            InputMode::Normal => "←→: switch tabs, c: create product",
            InputMode::CreateName => "Enter name, Enter: next field, Esc: cancel",
            InputMode::CreateDescription => "Enter description, Enter: save, Esc: cancel",
            _ => "←→: switch tabs",
        },
        Tab::Search => match app.input_mode {
            InputMode::Normal => {
                if app.has_multiple_panes() {
                    match app.active_pane {
                        ActivePane::Left => "Tab: edit selected, j/k: select, /: search",
                        ActivePane::Right => "Tab: back to results, Enter: save, ↑↓: fields",
                    }
                } else {
                    "j/k: select product, Enter: edit, /: search"
                }
            }
            InputMode::Search => "Type to search, Enter: confirm, Esc: cancel",
            InputMode::EditName => "Edit name, Tab: cancel, Enter: save, ↑↓: fields",
            InputMode::EditDescription => "Edit desc, Tab: cancel, Enter: save, ↑↓: fields",
            InputMode::EditProduction => "←→: toggle, Tab: cancel, Enter: save, ↑↓: fields",
            _ => "Tab: switch panes, j/k: navigate",
        },
        Tab::Inventory => match app.input_mode {
            InputMode::Normal => {
                if app.has_multiple_panes() {
                    match app.active_pane {
                        ActivePane::Left => "Tab: right pane, j/k: select product, /: search",
                        ActivePane::Right => "Tab: left pane, +/-: adjust stock, Enter: confirm",
                    }
                } else {
                    "j/k: select product, /: search"
                }
            }
            InputMode::InventorySearch => "Type to search inventory, Enter: confirm, Esc: cancel",
            _ => "Tab: switch panes, j/k: navigate",
        },
    };

    // Truncate status message if too long
    let max_status_len = 30;
    let truncated_status = if app.status_message.len() > max_status_len {
        format!("{}...", &app.status_message[..max_status_len.saturating_sub(3)])
    } else {
        app.status_message.clone()
    };

    let footer_text = format!("{} | {} | q:quit v{}", truncated_status, instructions, version);

    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(Color::Cyan))
        .wrap(Wrap { trim: false });

    f.render_widget(footer, area);
}