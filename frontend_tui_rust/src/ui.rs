use ratatui::{
    prelude::*,
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Tabs, Wrap},
};

// Style constants to reduce repetition
const HEADER_STYLE: Style = Style::new().fg(Color::Green).add_modifier(Modifier::BOLD);
const HELP_STYLE: Style = Style::new().fg(Color::Gray);
const NORMAL_STYLE: Style = Style::new().fg(Color::White);
const ACTIVE_BORDER_STYLE: Style = Style::new().fg(Color::Yellow).add_modifier(Modifier::BOLD);
const INACTIVE_BORDER_STYLE: Style = Style::new().fg(Color::White);

use crate::models::{ActivePane, InputMode, Tab};
use crate::state::App;

pub fn draw(f: &mut Frame, app: &mut App, version: &str) {
    let size = f.area();

    // Clear the frame
    f.render_widget(Clear, size);

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

    // Draw popup if in popup mode (global, works across all tabs)
    if matches!(
        app.input_mode,
        InputMode::NewCategory | InputMode::EditCategory | InputMode::NewTag | InputMode::EditTag
    ) {
        draw_popup(f, content_area, app);
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
    let is_creating = matches!(
        app.input_mode,
        InputMode::CreateName
            | InputMode::CreateDescription
            | InputMode::CreateCategory
            | InputMode::CreateProduction
            | InputMode::CreateTags
            | InputMode::CreateCategorySelect
            | InputMode::CreateTagSelect
    );
    let border_style = if is_creating {
        Style::default().fg(Color::Yellow).bold()
    } else {
        Style::default().fg(Color::White)
    };

    if is_creating {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        draw_create_left_pane(f, chunks[0], app, border_style);
        draw_create_right_pane(f, chunks[1], app);
    } else {
        let content = vec![Line::from("Press ENTER to create a new product")];
        let paragraph = Paragraph::new(content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Create Product")
                    .border_style(border_style),
            )
            .wrap(Wrap { trim: true });
        f.render_widget(paragraph, area);
    }
}

fn draw_create_left_pane(f: &mut Frame, area: Rect, app: &App, border_style: Style) {
    let mut content = vec![];

    // Name field
    let name_style = if matches!(app.input_mode, InputMode::CreateName) {
        Style::default().fg(Color::Yellow).bold()
    } else {
        Style::default().fg(Color::Cyan)
    };
    content.push(Line::from(vec![
        Span::styled("Name: ", name_style),
        Span::raw(&app.create_form.name),
        if matches!(app.input_mode, InputMode::CreateName) {
            Span::styled("_", Style::default().fg(Color::White))
        } else {
            Span::raw("")
        },
    ]));

    // Description field
    let desc_style = if matches!(app.input_mode, InputMode::CreateDescription) {
        Style::default().fg(Color::Yellow).bold()
    } else {
        Style::default().fg(Color::Cyan)
    };
    content.push(Line::from(vec![
        Span::styled("Description: ", desc_style),
        Span::raw(&app.create_form.description),
        if matches!(app.input_mode, InputMode::CreateDescription) {
            Span::styled("_", Style::default().fg(Color::White))
        } else {
            Span::raw("")
        },
    ]));

    // Category field
    let category_style = if matches!(
        app.input_mode,
        InputMode::CreateCategory | InputMode::CreateCategorySelect
    ) {
        Style::default().fg(Color::Yellow).bold()
    } else {
        Style::default().fg(Color::Cyan)
    };
    let category_display = if let Some(cat_id) = app.create_form.category_id {
        if let Some(category) = app.categories.iter().find(|c| c.id == Some(cat_id)) {
            format!("{} ({})", category.name, category.sku_initials)
        } else {
            format!("Category ID: {}", cat_id)
        }
    } else {
        "No category selected".to_string()
    };
    content.push(Line::from(vec![
        Span::styled("Category: ", category_style),
        Span::raw(category_display),
    ]));

    // Production field
    let prod_style = if matches!(app.input_mode, InputMode::CreateProduction) {
        Style::default().fg(Color::Yellow).bold()
    } else {
        Style::default().fg(Color::Cyan)
    };
    content.push(Line::from(vec![
        Span::styled("Production: ", prod_style),
        Span::raw(if matches!(app.input_mode, InputMode::CreateProduction) {
            format!(
                "[{}] Yes    [{}] No",
                if app.create_form.production { "x" } else { " " },
                if !app.create_form.production {
                    "x"
                } else {
                    " "
                }
            )
        } else {
            (if app.create_form.production { "Yes" } else { "No" }).to_string()
        }),
    ]));

    // Tags field
    let tags_style = if matches!(
        app.input_mode,
        InputMode::CreateTags | InputMode::CreateTagSelect | InputMode::EditTagSelect
    ) {
        Style::default().fg(Color::Yellow).bold()
    } else {
        NORMAL_STYLE
    };
    content.push(Line::from(vec![
        Span::styled("Tags: ", tags_style),
        Span::raw(if app.create_form.tags.is_empty() {
            "None".to_string()
        } else {
            app.create_form.tags.join(", ")
        }),
    ]));

    content.push(Line::from(""));
    let help_text = match app.input_mode {
        InputMode::CreateName => "[TAB/↓: Next] [↑: Prev] [ESC: Cancel]",
        InputMode::CreateDescription => "[TAB/↓: Next] [↑: Prev] [ESC: Cancel]",
        InputMode::CreateCategory => "[TAB: Select] [ESC: Cancel]",
        InputMode::CreateProduction => "[←→/y/n: Toggle] [TAB/↓: Next] [↑: Prev] [ESC: Cancel]",
        InputMode::CreateTags => "[TAB: Select Tags] [ENTER: Save] [↑: Prev] [ESC: Cancel]",
        _ => "[ENTER: Create] [ESC: Cancel]",
    };
    content.push(Line::from(vec![Span::styled(
        help_text,
        Style::default().fg(Color::Gray),
    )]));

    let paragraph = Paragraph::new(content)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Create Product")
                .border_style(border_style),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}

fn draw_create_right_pane(f: &mut Frame, area: Rect, app: &App) {
    let border_style = if matches!(app.active_pane, ActivePane::Right) {
        ACTIVE_BORDER_STYLE
    } else {
        INACTIVE_BORDER_STYLE
    };

    let mut content = vec![];

    match app.input_mode {
        InputMode::CreateCategorySelect => {
            content.push(Line::from(vec![Span::styled(
                "Categories:",
                Style::default().fg(Color::Green).bold(),
            )]));
            for (i, category) in app.categories.iter().enumerate() {
                let is_selected = i == app.create_form.category_selected_index;
                let style = if is_selected {
                    Style::default().fg(Color::Black).bg(Color::Yellow)
                } else {
                    Style::default().fg(Color::White)
                };
                content.push(Line::from(vec![Span::styled(
                    format!("{} ({})", category.name, category.sku_initials),
                    style,
                )]));
            }
            if app.categories.is_empty() {
                content.push(Line::from(vec![Span::styled(
                    "No categories available",
                    Style::default().fg(Color::Gray),
                )]));
            }
            content.push(Line::from(""));
            content.push(Line::from(vec![Span::styled(
                "[↑↓: Select] [ENTER: Choose] [n: New] [e: Edit] [ESC: Back]",
                Style::default().fg(Color::Gray),
            )]));
        }
        InputMode::CreateTagSelect => {
            content.extend(build_tag_selection_content(app, "Available Tags:", "[↑↓: Navigate] [Space: Select] [ENTER: Add Selected] [n: New] [e: Edit] [d: Delete] [ESC: Back]"));
        }
        _ => {
            content.push(Line::from(vec![Span::styled(
                "Select a field to see options",
                Style::default().fg(Color::Gray),
            )]));
        }
    }

    let paragraph = Paragraph::new(content)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Options")
                .border_style(border_style),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}

fn draw_popup(f: &mut Frame, area: Rect, app: &App) {
    let popup_area = centered_rect(80, 30, area);

    f.render_widget(Clear, popup_area);

    let mut content = vec![];

    match app.input_mode {
        InputMode::NewCategory | InputMode::EditCategory => {
            let name_style = if app.popup_field == 0 {
                Style::default().fg(Color::Yellow).bold()
            } else {
                Style::default().fg(Color::Cyan)
            };
            content.push(Line::from(vec![
                Span::styled("Category Name: ", name_style),
                Span::raw(&app.category_form.name),
                if app.popup_field == 0 {
                    Span::styled("_", Style::default().fg(Color::White))
                } else {
                    Span::raw("")
                },
            ]));
            let sku_style = if app.popup_field == 1 {
                Style::default().fg(Color::Yellow).bold()
            } else {
                Style::default().fg(Color::Cyan)
            };
            content.push(Line::from(vec![
                Span::styled("SKU (3 letters): ", sku_style),
                Span::raw(&app.category_form.sku),
                if app.popup_field == 1 {
                    Span::styled("_", Style::default().fg(Color::White))
                } else {
                    Span::raw("")
                },
            ]));
            let desc_style = if app.popup_field == 2 {
                Style::default().fg(Color::Yellow).bold()
            } else {
                Style::default().fg(Color::Cyan)
            };
            content.push(Line::from(vec![
                Span::styled("Description: ", desc_style),
                Span::raw(&app.category_form.description),
                if app.popup_field == 2 {
                    Span::styled("_", Style::default().fg(Color::White))
                } else {
                    Span::raw("")
                },
            ]));
        }
        InputMode::NewTag | InputMode::EditTag => {
            content.push(Line::from(vec![
                Span::styled("Tag Name: ", Style::default().fg(Color::Yellow).bold()),
                Span::raw(&app.tag_form.name),
                Span::styled("_", Style::default().fg(Color::White)),
            ]));
        }
        _ => {}
    }

    content.push(Line::from(""));
    content.push(Line::from(vec![Span::styled(
        "[ENTER: Save] [ESC: Cancel] (Tip: Use commas for multiple tags)",
        Style::default().fg(Color::Gray),
    )]));

    let title = match app.input_mode {
        InputMode::NewCategory => "New Category",
        InputMode::EditCategory => "Edit Category",
        InputMode::NewTag => "New Tag",
        InputMode::EditTag => "Edit Tag",
        _ => "Popup",
    };

    let paragraph = Paragraph::new(content)
        .block(Block::default().borders(Borders::ALL).title(title))
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, popup_area);
}

fn build_tag_selection_content<'a>(app: &'a App, header: &'a str, help: &'a str) -> Vec<Line<'a>> {
    let mut content = vec![];
    content.push(Line::from(vec![
        Span::styled(header, HEADER_STYLE),
    ]));
    for (i, tag) in app.tags.iter().enumerate() {
        let is_current = i == app.create_form.tag_selected_index;
        let is_selected = app.tag_selection.get(i).copied().unwrap_or(false);
        let marker = if is_selected { "[x]" } else { "[ ]" };
        let line = if is_current {
            format!("→ {} {}", marker, tag)
        } else {
            format!("  {} {}", marker, tag)
        };
        let style = NORMAL_STYLE;
        content.push(Line::from(Span::styled(line, style)));
    }
    if app.tags.is_empty() {
        content.push(Line::from(vec![
            Span::styled("No tags available", HELP_STYLE),
        ]));
    }
    content.push(Line::from(""));
    content.push(Line::from(vec![
        Span::styled(help, HELP_STYLE),
    ]));
    content
}

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

    // Search input - always show current query (no mode switching needed)
    let search_text = if search_query.is_empty() {
        "".to_string()
    } else {
        search_query.to_string()
    };
    let search_paragraph = Paragraph::new(search_text).block(
        Block::default()
            .borders(Borders::ALL)
            .title(title)
            .border_style(search_border_style),
    );
    f.render_widget(search_paragraph, chunks[0]);

    // Filter products based on search query
    let filtered_products: Vec<&crate::api::Product> = if search_query.is_empty() {
        app.products.iter().collect()
    } else {
        app.products
            .iter()
            .filter(|product| {
                product
                    .name
                    .to_lowercase()
                    .contains(&search_query.to_lowercase())
                    || product
                        .sku
                        .to_lowercase()
                        .contains(&search_query.to_lowercase())
            })
            .collect()
    };

    // Display results
    display_callback(f, chunks[1], app, &filtered_products, results_border_style);
}

/// Display callback for simple list format (used by Search tab)
fn display_as_list(
    f: &mut Frame,
    area: Rect,
    app: &App,
    products: &[&crate::api::Product],
    border_style: Style,
) {
    let mut content_lines = vec![];

    // Search instruction removed - now only in footer

    // Add the list items - always use filtered_selection_index for consistency
    for (i, product) in products.iter().enumerate() {
        let style = if i == app.filtered_selection_index {
            Style::default().fg(Color::Yellow)  // Yellow text for selected item
        } else {
            Style::default().fg(Color::White)   // White text for unselected items
        };

        content_lines.push(Line::from(Span::styled(
            format!(
                "{} - {} ({})",
                product.sku,
                product.name,
                if product.production {
                    "Production"
                } else {
                    "Prototype"
                }
            ),
            style,
        )));
    }

    let paragraph = Paragraph::new(content_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Results")
                .border_style(border_style),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}

/// Display callback for table format (used by Inventory tab)
fn display_as_table(
    f: &mut Frame,
    area: Rect,
    app: &App,
    products: &[&crate::api::Product],
    border_style: Style,
) {
    // Header
    let header = vec![Line::from(
        "SKU         Name                    Qty   Price   Status",
    )];

    // Product rows
    let mut rows = vec![];
    
    for (i, product) in products.iter().enumerate() {
        let style = if i == app.filtered_selection_index {
            Style::default().fg(Color::Yellow)  // Yellow text for selected item
        } else {
            Style::default().fg(Color::White)   // White text for unselected items
        };

        // Mock inventory data for now (in real app, this would come from API)
        let qty = 10 - (i as i32 * 2); // Mock data
        let price = 25.50 + (i as f64 * 5.0); // Mock data
        let status = if qty > 5 {
            "In Stock"
        } else if qty > 0 {
            "Low Stock"
        } else {
            "Out of Stock"
        };

        rows.push(
            Line::from(format!(
                "{:<10} {:<20} {:>5} ${:>6.2} {:<10}",
                product.sku,
                &product.name[..product.name.len().min(20)],
                qty,
                price,
                status
            ))
            .style(style),
        );
    }

    let content = [header, rows].concat();

    let paragraph = Paragraph::new(content)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Inventory Results")
                .border_style(border_style),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}

fn draw_search_left_pane(f: &mut Frame, area: Rect, app: &App) {
    let search_border_style = if matches!(app.active_pane, ActivePane::Left) {
        Style::default().fg(Color::Yellow).bold()
    } else {
        Style::default().fg(Color::White)
    };

    let results_border_style = if matches!(app.active_pane, ActivePane::Left) {
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
        search_border_style,
        results_border_style,
        display_as_list,
    );
}

fn draw_search_right_pane(f: &mut Frame, area: Rect, app: &App) {
    let border_style = if matches!(app.active_pane, ActivePane::Right) {
        ACTIVE_BORDER_STYLE
    } else {
        INACTIVE_BORDER_STYLE
    };

    if matches!(app.input_mode, InputMode::EditTagSelect) {
        // Draw tag selection
        let content = build_tag_selection_content(
            app,
            "Available Tags:",
            "[↑↓: Navigate] [Space: Select] [ENTER: Add Selected] [n: New] [e: Edit] [d: Delete] [ESC: Back]",
        );
        let paragraph = Paragraph::new(content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Tag Selection")
                    .border_style(border_style),
            )
            .wrap(Wrap { trim: true });
        f.render_widget(paragraph, area);
    } else if let Some(product) = app.products.get(app.selected_index) {
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

        let tags_style = if matches!(app.input_mode, InputMode::EditTags) {
            Style::default().fg(Color::Yellow).bold()
        } else {
            Style::default().fg(Color::Cyan)
        };

        let tags_text = product.tags.join(", ");

let content = vec![
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
                    format!(
                        "[{}] Yes    [{}] No",
                        if product.production { "x" } else { " " },
                        if !product.production { "x" } else { " " }
                    )
                } else {
                    (if product.production { "Yes" } else { "No" }).to_string()
                }),
            ]),
            Line::from(vec![
                Span::styled("Tags: ", tags_style),
                Span::raw(&tags_text),
            ]),
            Line::from(""),
        ];

        let paragraph = Paragraph::new(content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Product Details")
                    .border_style(border_style),
            )
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, area);
    } else {
        let paragraph = Paragraph::new("No product selected").block(
            Block::default()
                .borders(Borders::ALL)
                .title("Product Details")
                .border_style(border_style),
        );
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
    let search_border_style = if matches!(app.active_pane, ActivePane::Left) {
        Style::default().fg(Color::Yellow).bold()
    } else {
        Style::default().fg(Color::White)
    };

    let results_border_style = if matches!(app.active_pane, ActivePane::Left) {
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
        search_border_style,
        results_border_style,
        display_as_table,
    );
}

fn draw_inventory_right_pane(f: &mut Frame, area: Rect, app: &App) {
    let border_style = if matches!(app.active_pane, ActivePane::Right) {
        ACTIVE_BORDER_STYLE
    } else {
        INACTIVE_BORDER_STYLE
    };

if let Some(product) = app.products.get(app.filtered_selection_index) {
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
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Stock Adjustment")
                    .border_style(border_style),
            )
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, area);
    } else {
        let paragraph = Paragraph::new("No product selected").block(
            Block::default()
                .borders(Borders::ALL)
                .title("Stock Adjustment")
                .border_style(border_style),
        );
        f.render_widget(paragraph, area);
    }
}

fn draw_inventory_totals(f: &mut Frame, area: Rect, app: &App) {
    // Mock totals - in real app, this would be calculated from API data
    let total_products = app.products.len();
    let total_value = 1250.75; // Mock data
    let low_stock_items = 3; // Mock data

    let content = format!(
        "Total Products: {} | Total Value: ${:.2} | Low Stock Items: {}",
        total_products, total_value, low_stock_items
    );

    let paragraph = Paragraph::new(content)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Inventory Summary"),
        )
        .style(Style::default().fg(Color::Green));

    f.render_widget(paragraph, area);
}

fn draw_footer(f: &mut Frame, area: Rect, app: &App, version: &str) {
    // Get instructions based on current tab, pane, and mode
    let instructions = match app.current_tab {
        Tab::Create => match app.input_mode {
            InputMode::Normal => "←→: switch tabs, Enter: create product",
            InputMode::CreateName => "Enter name, Tab/↓: next, ↑: prev, Esc: cancel",
            InputMode::CreateDescription => "Enter desc, Tab/↓: next, ↑: prev, Esc: cancel",
            InputMode::CreateCategory => "Tab: Select, Esc: cancel",
            InputMode::CreateCategorySelect => {
                "↑↓: select, Enter: choose, n: new, e: edit, Esc: back"
            }
            InputMode::CreateProduction => "←→/y/n: toggle, Tab/↓: next, ↑: prev, Esc: cancel",
            InputMode::CreateTags => "Tab: Select Tags, Enter: save, ↑: prev, Esc: cancel",
            InputMode::CreateTagSelect => "↑↓: select, Enter: choose, n: new, e: edit, Esc: back",
            InputMode::NewCategory | InputMode::EditCategory => {
                "Enter name, Enter: save, Esc: cancel"
            }
            InputMode::NewTag | InputMode::EditTag => "Enter name, Enter: save, Esc: cancel",
            _ => "←→: switch tabs",
        },
        Tab::Search => match app.input_mode {
            InputMode::Normal => {
                if app.has_multiple_panes() {
                    match app.active_pane {
                        ActivePane::Left => "Tab: edit selected, j/k: select, type: search",
                        ActivePane::Right => "Tab: back to results, Enter: save, ↑↓: fields",
                    }
                } else {
                    "j/k: select product, Enter: edit, type: search"
                }
            }
            InputMode::EditName => "Edit name, Tab: cancel, Enter: save, ↑↓: fields",
            InputMode::EditDescription => "Edit desc, Tab: cancel, Enter: save, ↑↓: fields",
            InputMode::EditProduction => "←→: toggle, Tab: cancel, Enter: save, ↑↓: fields",
            _ => "Tab: switch panes, j/k: navigate",
        },
        Tab::Inventory => match app.input_mode {
            InputMode::Normal => {
                if app.has_multiple_panes() {
                    match app.active_pane {
                        ActivePane::Left => "Tab: right pane, j/k: select product, type: search",
                        ActivePane::Right => "Tab: left pane, +/-: adjust stock, Enter: confirm",
                    }
                } else {
                    "j/k: select product, type: search"
                }
            }
            _ => "Tab: switch panes, j/k: navigate",
        },
    };

    // Truncate status message if too long
    let max_status_len = 30;
    let truncated_status = if app.status_message.len() > max_status_len {
        format!(
            "{}...",
            &app.status_message[..max_status_len.saturating_sub(3)]
        )
    } else {
        app.status_message.clone()
    };

    let footer_text = format!(
        "{} | {} | q:quit v{}",
        truncated_status, instructions, version
    );

    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(Color::Cyan))
        .wrap(Wrap { trim: false });

    f.render_widget(footer, area);
}
