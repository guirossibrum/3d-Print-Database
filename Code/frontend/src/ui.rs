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

use crate::models::{ActivePane, InputMode, ItemType, Tab};
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
    let is_popup_active = matches!(
        app.input_mode,
        InputMode::NewCategory
            | InputMode::EditCategory
            | InputMode::NewTag
            | InputMode::NewMaterial
            | InputMode::DeleteConfirm
            | InputMode::DeleteFileConfirm
    );

    if is_popup_active {
        draw_popup(f, content_area, app);
    }

    // Draw footer (only when no popup is active)
    if !is_popup_active {
        draw_footer(f, chunks[4], app, version);
    }
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
        InputMode::EditName
            | InputMode::EditDescription
            | InputMode::EditProduction
            | InputMode::EditTags
            | InputMode::EditMaterials
            | InputMode::EditSelect
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

        draw_product_form(f, chunks[0], app, border_style);
        draw_selection_pane(f, chunks[1], app, border_style);
    } else {
        let content = vec![Line::from("Press 'n' to create a new product")];
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

fn draw_product_form(f: &mut Frame, area: Rect, app: &App, border_style: Style) {
    let product = &app.current_product;

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

    let category_style = if matches!(app.input_mode, InputMode::EditCategory) ||
        (matches!(app.input_mode, InputMode::EditSelect) &&
         matches!(app.selection_type, Some(crate::models::SelectionType::Category))) {
        Style::default().fg(Color::Yellow).bold()
    } else {
        Style::default().fg(Color::Cyan)
    };

    let tags_style = if matches!(app.input_mode, InputMode::EditTags) {
        Style::default().fg(Color::Yellow).bold()
    } else {
        Style::default().fg(Color::Cyan)
    };

    let materials_style = if matches!(app.input_mode, InputMode::EditMaterials) ||
        (matches!(app.input_mode, InputMode::EditSelect) &&
         matches!(app.selection_type, Some(crate::models::SelectionType::Material))) {
        Style::default().fg(Color::Yellow).bold()
    } else {
        Style::default().fg(Color::Cyan)
    };

    let category_name = if product.id.is_none() {
        // Create mode - show selectable category
        app
            .categories
            .iter()
            .find(|c| c.id == product.category_id)
            .map(|c| c.name.clone())
            .unwrap_or_else(|| "No category selected".to_string())
    } else {
        // Edit mode - show fixed category
        app
            .categories
            .iter()
            .find(|c| c.id == product.category_id)
            .map(|c| format!("{} (Fixed)", c.name))
            .unwrap_or_else(|| "Unknown (Fixed)".to_string())
    };

    let tags_text = product.tags.join(", ");
    let materials_text = product
        .material
        .as_ref()
        .map(|m| m.join(", "))
        .unwrap_or_else(|| "None".to_string());

    let content = vec![
        Line::from(vec![
            Span::styled("SKU: ", Style::default().fg(Color::White)),
            Span::raw(&product.sku),
        ]),
        Line::from(vec![
            Span::styled("Category: ", category_style),
            Span::raw(category_name),
            if product.id.is_none() && matches!(app.input_mode, InputMode::EditCategory) {
                Span::styled("_", Style::default().fg(Color::White))
            } else {
                Span::raw("")
            },
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
                    if !product.production {
                        "x"
                    } else {
                        " "
                    }
                )
            } else {
                (if product.production { "Yes" } else { "No" }).to_string()
            }),
        ]),
        Line::from(vec![
            Span::styled("Tags: ", tags_style),
            Span::raw(if tags_text.is_empty() { "None" } else { &tags_text }),
        ]),
        Line::from(vec![
            Span::styled("Materials: ", materials_style),
            Span::raw(&materials_text),
        ]),
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
}

fn draw_selection_pane(f: &mut Frame, area: Rect, app: &App, border_style: Style) {
    if matches!(app.input_mode, InputMode::EditSelect) && matches!(app.selection_type, Some(crate::models::SelectionType::Tag)) {
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
    } else if matches!(app.input_mode, InputMode::EditSelect) && matches!(app.selection_type, Some(crate::models::SelectionType::Material)) {
        // Draw material selection
        let content = build_material_selection_content(
            app,
            "Available Materials:",
            "[↑↓: Navigate] [Space: Select] [ENTER: Add Selected] [n: New] [d: Delete] [ESC: Back]",
        );
        let paragraph = Paragraph::new(content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Material Selection")
                    .border_style(border_style),
            )
            .wrap(Wrap { trim: true });
        f.render_widget(paragraph, area);
    }
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
        InputMode::NewTag | InputMode::NewMaterial => {
            let item_type_name = match app.item_type {
                ItemType::Tag => "Tag",
                ItemType::Material => "Material",
                ItemType::Category => "Category",
            };

            content.push(Line::from(vec![
                Span::styled(
                    format!("{} Name: ", item_type_name),
                    Style::default().fg(Color::Yellow).bold(),
                ),
                Span::raw(&app.item_form.name),
                Span::styled("_", Style::default().fg(Color::White)),
            ]));

            content.push(Line::from(vec![Span::styled(
                format!(
                    "[Enter] Save     [ESC] Cancel    (Tip: Use commas for multiple {})",
                    item_type_name.to_lowercase()
                ),
                Style::default().fg(Color::Gray),
            )]));
        }
        InputMode::EditTag => {
            content.push(Line::from(vec![
                Span::styled("Tag Name: ", Style::default().fg(Color::Yellow).bold()),
                Span::raw(&app.item_form.name),
                Span::styled("_", Style::default().fg(Color::White)),
            ]));
        }
        InputMode::DeleteConfirm => {
            if let Some(product) = &app.selected_product_for_delete {
                content.push(Line::from(vec![
                    Span::styled("Delete Product: ", Style::default().fg(Color::Red).bold()),
                    Span::raw(&product.name),
                ]));
                content.push(Line::from(vec![
                    Span::styled("SKU: ", Style::default().fg(Color::Cyan)),
                    Span::raw(&product.sku),
                ]));
                content.push(Line::from(""));
                content.push(Line::from(vec![Span::styled(
                    "Choose deletion method:",
                    Style::default().fg(Color::Yellow),
                )]));
                content.push(Line::from(""));
                let option1_style = if app.delete_option == 0 {
                    Style::default().fg(Color::Yellow).bold()
                } else {
                    Style::default().fg(Color::White)
                };
                let option2_style = if app.delete_option == 1 {
                    Style::default().fg(Color::Yellow).bold()
                } else {
                    Style::default().fg(Color::White)
                };
                content.push(Line::from(vec![Span::styled(
                    " [1] Delete record from database only",
                    option1_style,
                )]));
                content.push(Line::from(vec![Span::styled(
                    " [2] Delete record and all associated files",
                    option2_style,
                )]));
            }
        }
        InputMode::DeleteFileConfirm => {
            if let Some(product) = &app.selected_product_for_delete {
                content.push(Line::from(vec![
                    Span::styled("Delete Files for: ", Style::default().fg(Color::Red).bold()),
                    Span::raw(&product.name),
                ]));
                content.push(Line::from(vec![
                    Span::styled("SKU: ", Style::default().fg(Color::Cyan)),
                    Span::raw(&product.sku),
                ]));
                content.push(Line::from(""));
                content.push(Line::from(vec![Span::styled(
                    "Files to be deleted:",
                    Style::default().fg(Color::Yellow),
                )]));
                content.push(Line::from(""));

                // Display file tree
                for line in &app.file_tree_content {
                    content.push(Line::from(vec![Span::raw(line)]));
                }

                content.push(Line::from(""));
                content.push(Line::from(vec![
                    Span::styled("Confirm deletion? ", Style::default().fg(Color::Red).bold()),
                    Span::raw("[y] Yes [n] No"),
                ]));
            }
        }
        _ => {}
    }

    content.push(Line::from(""));
    let help_text = match app.input_mode {
        InputMode::DeleteConfirm => "[ENTER] Confirm     [ESC] Cancel     [1/2] Select option",
        InputMode::DeleteFileConfirm => "[y] Confirm deletion     [n] Cancel     [ESC] Back",
        _ => "", // No generic fallback - specific cases handle their own help
    };

    // Only add help text if it's not empty
    if !help_text.is_empty() {
        content.push(Line::from(vec![Span::styled(
            help_text,
            Style::default().fg(Color::Gray),
        )]));
    }

    let title = match app.input_mode {
        InputMode::NewCategory => "New Category",
        InputMode::EditCategory => "Edit Category",
        InputMode::NewTag => "New Tag",
        InputMode::EditTag => "Edit Tag",
        InputMode::DeleteConfirm => "Delete Product",
        InputMode::DeleteFileConfirm => "Confirm File Deletion",
        _ => "Popup",
    };

    let paragraph = Paragraph::new(content)
        .block(Block::default().borders(Borders::ALL).title(title))
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, popup_area);
}

fn build_tag_selection_content<'a>(app: &'a App, header: &'a str, help: &'a str) -> Vec<Line<'a>> {
    let mut content = vec![];
    content.push(Line::from(vec![Span::styled(header, HEADER_STYLE)]));
    for (i, tag) in app.tags.iter().enumerate() {
        let is_current = i == app.tag_selected_index;
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
        content.push(Line::from(vec![Span::styled(
            "No tags available",
            HELP_STYLE,
        )]));
    }
    content.push(Line::from(""));
    content.push(Line::from(vec![Span::styled(help, HELP_STYLE)]));
    content
}

#[allow(dead_code)]
fn build_category_selection_content<'a>(app: &'a App, header: &'a str, help: &'a str) -> Vec<Line<'a>> {
    let mut content = vec![];
    content.push(Line::from(vec![
        Span::styled(header, HEADER_STYLE),
    ]));
    for (i, category) in app.categories.iter().enumerate() {
        let is_current = i == app.selected_category_index;
        let is_selected = app.category_selection.get(i).copied().unwrap_or(false);
        let marker = if is_selected { "[x]" } else { "[ ]" };
        let line = if is_current {
            format!("→ {} {} ({})", marker, category.name, category.sku_initials)
        } else {
            format!("  {} {} ({})", marker, category.name, category.sku_initials)
        };
        let style = NORMAL_STYLE;
        content.push(Line::from(Span::styled(line, style)));
    }
    if app.categories.is_empty() {
        content.push(Line::from(vec![
            Span::styled("No categories available", HELP_STYLE),
        ]));
    }
    content.push(Line::from(""));
    content.push(Line::from(vec![
        Span::styled(help, HELP_STYLE),
    ]));
    content
}

fn build_material_selection_content<'a>(
    app: &'a App,
    header: &'a str,
    help: &'a str,
) -> Vec<Line<'a>> {
    let mut content = vec![];
    content.push(Line::from(vec![Span::styled(header, HEADER_STYLE)]));
    for (i, material) in app.materials.iter().enumerate() {
        let is_current = i == app.material_selected_index;
        let is_selected = app.tag_selection.get(i).copied().unwrap_or(false);
        let marker = if is_selected { "[x]" } else { "[ ]" };
        let line = if is_current {
            format!("→ {} {}", marker, material)
        } else {
            format!("  {} {}", marker, material)
        };
        let style = NORMAL_STYLE;
        content.push(Line::from(Span::styled(line, style)));
    }
    if app.materials.is_empty() {
        content.push(Line::from(vec![Span::styled(
            "No materials available",
            HELP_STYLE,
        )]));
    }
    content.push(Line::from(""));
    content.push(Line::from(vec![Span::styled(help, HELP_STYLE)]));
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
#[allow(clippy::too_many_arguments)]
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
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(10),   // Content area
            Constraint::Length(1), // Footer helper
            Constraint::Length(1), // Status message line
            Constraint::Length(1), // Version info
        ])
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

    // Add the list items - use product ID for selection
    for (i, product) in products.iter().enumerate() {
        let style = if product.id == app.selected_product_id
            || (app.selected_product_id.is_none() && i == 0)
        {
            Style::default().fg(Color::Yellow) // Yellow text for selected item
        } else {
            Style::default().fg(Color::White) // White text for unselected items
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
        let style = if product.id == app.selected_product_id
            || (app.selected_product_id.is_none() && i == 0)
        {
            Style::default().fg(Color::Yellow) // Yellow text for selected item
        } else {
            Style::default().fg(Color::White) // White text for unselected items
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

    if matches!(app.input_mode, InputMode::EditSelect) && matches!(app.selection_type, Some(crate::models::SelectionType::Tag)) {
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
    } else if matches!(app.input_mode, InputMode::EditSelect) && matches!(app.selection_type, Some(crate::models::SelectionType::Category)) {
        // Draw category selection
        let content = build_category_selection_content(
            app,
            "Available Categories:",
            "[↑↓: Navigate] [Space: Select] [ENTER: Select Category] [ESC: Back]",
        );
        let paragraph = Paragraph::new(content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Category Selection")
                    .border_style(border_style),
            )
            .wrap(Wrap { trim: true });
        f.render_widget(paragraph, area);
    } else if matches!(app.input_mode, InputMode::EditSelect) && matches!(app.selection_type, Some(crate::models::SelectionType::Material)) {
        // Draw material selection
        let content = build_material_selection_content(
            app,
            "Available Materials:",
            "[↑↓: Navigate] [Space: Select] [ENTER: Add Selected] [n: New] [d: Delete] [ESC: Back]",
        );
        let paragraph = Paragraph::new(content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Material Selection")
                    .border_style(border_style),
            )
            .wrap(Wrap { trim: true });
        f.render_widget(paragraph, area);
    } else if matches!(app.current_tab, Tab::Create) || (matches!(app.current_tab, Tab::Search | Tab::Inventory) && matches!(app.input_mode, InputMode::EditName | InputMode::EditDescription | InputMode::EditProduction | InputMode::EditTags | InputMode::EditMaterials)) {
        // Use unified product form for both Create and Edit tabs
        draw_product_form(f, area, app, border_style);
    } else if matches!(app.current_tab, Tab::Search | Tab::Inventory) && matches!(app.input_mode, InputMode::EditName | InputMode::EditDescription | InputMode::EditProduction | InputMode::EditTags | InputMode::EditMaterials | InputMode::EditSelect) {
        // Only show product details when actually editing in Search/Inventory tabs
        if let Some(product) = app.get_selected_product() {
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

            let materials_style = if matches!(app.input_mode, InputMode::EditMaterials) ||
                (matches!(app.input_mode, InputMode::EditSelect) &&
                 matches!(app.selection_type, Some(crate::models::SelectionType::Material))) {
                Style::default().fg(Color::Yellow).bold()
            } else {
                Style::default().fg(Color::Cyan)
            };

            let category_name = app
                .categories
                .iter()
                .find(|c| c.id == product.category_id)
                .map(|c| c.name.as_str())
                .unwrap_or("Unknown");

            let tags_text = product.tags.join(", ");
            let materials_text = product
                .material
                .as_ref()
                .map(|m| m.join(", "))
                .unwrap_or_else(|| "None".to_string());

            let content = vec![
                Line::from(vec![
                    Span::styled("SKU: ", Style::default().fg(Color::White)),
                    Span::raw(&product.sku),
                ]),
                Line::from(vec![
                    Span::styled("Category: ", Style::default().fg(Color::White)),
                    Span::raw(category_name),
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
                Line::from(vec![
                    Span::styled("Materials: ", materials_style),
                    Span::raw(&materials_text),
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

    if let Some(product) = app.get_selected_product() {
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
            InputMode::Normal => "[←/→] switch tabs     [n] create product",
            InputMode::EditName => "[Enter] name     [↑/↓] select     [Esc] cancel",
            InputMode::EditDescription => "[Enter] desc     [↑/↓] select     [Esc] cancel",
            InputMode::EditProduction => {
                "[←/→] toggle     [y/n] toggle     [↑/↓] select     [Esc] cancel"
            }
            InputMode::EditTags => {
                "[Tab] select tags     [Enter] save     [↑] prev     [Esc] cancel"
            }
            InputMode::NewCategory | InputMode::EditCategory => {
                "[Enter] name     [Enter] save     [Esc] cancel"
            }
            InputMode::NewTag | InputMode::NewMaterial => {
                "[Enter] name     [Enter] save     [Esc] cancel"
            }
            InputMode::EditTag => "[Enter] name     [Enter] save     [Esc] cancel",
            _ => "[←/→] switch tabs",
        },
        Tab::Search => match app.input_mode {
            InputMode::Normal => {
                if app.has_multiple_panes() {
                    match app.active_pane {
                        ActivePane::Left => {
                            "[Tab] edit selected     [↑/↓] select     [type] search     [Ctrl+o] open folder     [Ctrl+d] delete"
                        }
                        ActivePane::Right => {
                            "[Tab] back to results     [Enter] save     [↑/↓] select"
                        }
                    }
                } else {
                    "[↑/↓] select product     [Enter] edit       [type] search     [Ctrl+o] open folder      [Ctrl+d] delete"
                }
            }
            InputMode::EditName => {
                "[Enter] name     [Tab] cancel     [Enter] save     [↑/↓] select"
            }
            InputMode::EditDescription => {
                "[Enter] desc      [Tab] cancel     [Enter] save     [↑/↓] select"
            }
            InputMode::EditProduction => {
                "[←/→] toggle     [Tab] cancel     [Enter] save     [↑/↓] select"
            }

            InputMode::EditMaterials => "[Tab] select materials     [Enter] save     [↑/↓] select",
            _ => "[Tab] switch panes     [↑/↓] navigate",
        },
        Tab::Inventory => match app.input_mode {
            InputMode::Normal => {
                if app.has_multiple_panes() {
                    match app.active_pane {
                        ActivePane::Left => {
                            "[Tab] right pane     [↑/↓] select     [type] search     [Ctrl+d] delete"
                        }
                        ActivePane::Right => {
                            "[Tab] left pane     [+/-] adjust stock     [Enter] confirm"
                        }
                    }
                } else {
                    "[↑/↓] select product     [type] search     [Ctrl+d] delete"
                }
            }
            _ => "[Tab] switch panes     [↑/↓] navigate",
        },
    };

    // Truncate status message if too long
    let max_status_len = 30;
    let _truncated_status = if app.status_message.len() > max_status_len {
        format!(
            "{}...",
            &app.status_message[..max_status_len.saturating_sub(3)]
        )
    } else {
        app.status_message.clone()
    };

    // Split footer area into helper + status + version
    let footer_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Helper text
            Constraint::Length(1), // Status message
            Constraint::Length(1), // Version
        ])
        .split(area);

    // Helper text
    let helper = Paragraph::new(instructions)
        .style(Style::default().fg(Color::Cyan))
        .wrap(Wrap { trim: false });
    f.render_widget(helper, footer_chunks[0]);

    // Status message
    if app.should_show_status() && !app.status_message.is_empty() {
        let status_style = if app.status_message.starts_with("Error") {
            Style::default().fg(Color::Red).bold()
        } else {
            Style::default().fg(Color::Green)
        };

        let status = Paragraph::new(app.status_message.as_str())
            .style(status_style)
            .wrap(Wrap { trim: false });
        f.render_widget(status, footer_chunks[1]);
    }

    // Version info
    let version_text = format!("3D Print Database TUI v{}", version);
    let version_para = Paragraph::new(version_text)
        .style(Style::default().fg(Color::Gray))
        .alignment(ratatui::layout::Alignment::Right);
    f.render_widget(version_para, footer_chunks[2]);
}
