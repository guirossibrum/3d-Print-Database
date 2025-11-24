"""
UI rendering for the 3D Print Database TUI
"""

import curses
from typing import List, Dict, Any
from app import App, Tab


def render_ui(stdscr, app: App):
    """Main UI rendering function"""
    height, width = stdscr.getmaxyx()

    # Clear the entire screen and set background
    stdscr.clear()
    if curses.has_colors():
        stdscr.bkgd(" ", curses.color_pair(6))  # Set entire screen background

    # Render header
    render_header(stdscr, app, width)

    # Render tabs
    render_tabs(stdscr, app, width)

    # Render main content area
    content_start_y = app.config.layout["header_height"] + 1
    content_height = height - content_start_y - app.config.layout["footer_height"]

    if app.current_tab == Tab.CREATE:
        render_create_tab(stdscr, app, content_start_y, content_height, width)
    elif app.current_tab == Tab.SEARCH:
        render_search_tab(stdscr, app, content_start_y, content_height, width)
    elif app.current_tab == Tab.INVENTORY:
        render_inventory_tab(stdscr, app, content_start_y, content_height, width)

    # Render dialog if shown
    if app.show_dialog:
        render_dialog(stdscr, app, height, width)

    # Render footer
    render_footer(stdscr, app, height, width)

    # Refresh screen
    stdscr.refresh()


def render_header(stdscr, app: App, width: int):
    """Render the application header"""
    header_text = "3D Print Database TUI"
    centered_x = (width - len(header_text)) // 2

    # Use color pair 1 for header (white text on terminal background)
    if curses.has_colors():
        stdscr.attron(curses.color_pair(app.config.colors["header"]))

    stdscr.addstr(0, centered_x, header_text)

    if curses.has_colors():
        stdscr.attroff(curses.color_pair(app.config.colors["header"]))

    # Status line - use appropriate color for status type
    if app.status_message:
        status_color = app.config.colors.get(
            app.status_type, app.config.colors["default"]
        )
        if curses.has_colors():
            stdscr.attron(curses.color_pair(status_color))
        stdscr.addstr(1, 0, f"[{app.status_type.upper()}] {app.status_message}")
        if curses.has_colors():
            stdscr.attroff(curses.color_pair(status_color))


def render_tabs(stdscr, app: App, width: int):
    """Render the tab navigation"""
    tab_y = app.config.layout["header_height"]

    tab_names = ["Create", "Search", "Inventory"]
    tab_width = width // len(tab_names)

    for i, tab_name in enumerate(tab_names):
        start_x = i * tab_width
        is_selected = i == app.selected_tab_index

        # Use accent color for selected tab, default for others
        if is_selected and curses.has_colors():
            stdscr.attron(curses.color_pair(app.config.colors["accent"]))
        elif curses.has_colors():
            stdscr.attron(curses.color_pair(app.config.colors["default"]))

        # Truncate tab name if too long
        display_name = (
            tab_name[: tab_width - 3] + "..."
            if len(tab_name) > tab_width - 2
            else tab_name
        )
        stdscr.addstr(tab_y, start_x, f" {display_name} ")

        if curses.has_colors():
            stdscr.attroff(curses.A_COLOR)


def render_create_tab(stdscr, app: App, start_y: int, height: int, width: int):
    """Render the create product tab"""
    # Apply default text color
    if curses.has_colors():
        stdscr.attron(curses.color_pair(app.config.colors["default"]))

    stdscr.addstr(start_y, 0, "Create New Product")
    stdscr.addstr(start_y + 1, 0, "=" * min(width, 50))  # Limit line length

    # Form fields
    y = start_y + 3
    stdscr.addstr(y, 0, "Name:")
    stdscr.addstr(y, 10, app.create_form["name"] or "[Enter product name]")

    y += 1
    stdscr.addstr(y, 0, "Description:")
    stdscr.addstr(y, 15, app.create_form["description"] or "[Enter description]")

    y += 1
    category_name = "None"
    if app.create_form["category_id"]:
        category = next(
            (c for c in app.categories if c["id"] == app.create_form["category_id"]),
            None,
        )
        if category:
            category_name = category["name"]
    stdscr.addstr(y, 0, f"Category: {category_name}")

    y += 1
    tags_str = ", ".join(app.create_form["tags"]) if app.create_form["tags"] else "None"
    stdscr.addstr(y, 0, f"Tags: {tags_str}")

    y += 1
    production_status = (
        "Production Ready" if app.create_form["production"] else "Prototype"
    )
    stdscr.addstr(y, 0, f"Status: {production_status}")

    # Instructions
    y += 3
    if curses.has_colors():
        stdscr.attron(curses.color_pair(app.config.colors["accent"]))
    stdscr.addstr(y, 0, "Press 'c' to create product, 'q' to quit")
    if curses.has_colors():
        stdscr.attroff(curses.color_pair(app.config.colors["accent"]))

    # Turn off default color
    if curses.has_colors():
        stdscr.attroff(curses.color_pair(app.config.colors["default"]))


def render_search_tab(stdscr, app: App, start_y: int, height: int, width: int):
    """Render the search/edit tab"""
    # Apply default text color
    if curses.has_colors():
        stdscr.attron(curses.color_pair(app.config.colors["default"]))

    stdscr.addstr(start_y, 0, f"Search Products (Query: '{app.search_query}')")
    stdscr.addstr(start_y + 1, 0, "=" * min(width, 60))  # Limit line length

    # Search results
    results_y = start_y + 3
    max_results = min(len(app.search_results), height - 8)

    for i in range(max_results):
        product = app.search_results[i]
        is_selected = i == app.selected_product_index

        if is_selected and curses.has_colors():
            stdscr.attron(curses.color_pair(app.config.colors["selected"]))
        elif curses.has_colors():
            stdscr.attron(curses.color_pair(app.config.colors["default"]))

        sku = product.get("sku", "N/A")
        name = product.get("name", "N/A")
        status = "Production" if product.get("production") else "Prototype"

        line = f"{i + 1:2d}. {sku} - {name} ({status})"
        stdscr.addstr(results_y + i, 0, line[: width - 1])

        # Reset color after each line
        if curses.has_colors():
            stdscr.attroff(curses.A_COLOR)

    # Show if there are more results
    if max_results < len(app.search_results):
        if curses.has_colors():
            stdscr.attron(curses.color_pair(app.config.colors["accent"]))
        stdscr.addstr(
            results_y + max_results,
            0,
            f"... and {len(app.search_results) - max_results} more",
        )
        if curses.has_colors():
            stdscr.attroff(curses.color_pair(app.config.colors["accent"]))

    # Instructions
    instructions_y = results_y + max_results + 2
    if curses.has_colors():
        stdscr.attron(curses.color_pair(app.config.colors["accent"]))
    stdscr.addstr(
        instructions_y,
        0,
        "↑/↓: Navigate  Enter: Select  e: Edit  d: Delete  /: Search  q: Quit",
    )
    if curses.has_colors():
        stdscr.attroff(curses.color_pair(app.config.colors["accent"]))

    # Turn off default color
    if curses.has_colors():
        stdscr.attroff(curses.color_pair(app.config.colors["default"]))


def render_inventory_tab(stdscr, app: App, start_y: int, height: int, width: int):
    """Render the inventory management tab"""
    # Apply default text color
    if curses.has_colors():
        stdscr.attron(curses.color_pair(app.config.colors["default"]))

    stdscr.addstr(start_y, 0, "Inventory Management")
    stdscr.addstr(start_y + 1, 0, "=" * min(width, 50))  # Limit line length

    # Inventory list
    inventory_y = start_y + 3
    max_items = min(len(app.inventory), height - 8)

    # Header
    if curses.has_colors():
        stdscr.attron(curses.color_pair(app.config.colors["accent"]))
    header = "SKU         Name                    Stock  Reorder Status"
    stdscr.addstr(inventory_y, 0, header[: width - 1])
    if curses.has_colors():
        stdscr.attroff(curses.color_pair(app.config.colors["accent"]))

    for i in range(max_items):
        item = app.inventory[i]
        is_selected = i == app.selected_product_index

        if is_selected and curses.has_colors():
            stdscr.attron(curses.color_pair(app.config.colors["selected"]))
        elif curses.has_colors():
            stdscr.attron(curses.color_pair(app.config.colors["default"]))

        sku = item.get("sku", "N/A")[:10]
        name = item.get("name", "N/A")[:20]
        stock = str(item.get("stock_quantity", 0))
        reorder = str(item.get("reorder_point", 0))
        status = item.get("status", "unknown")

        line = f"{sku:<10} {name:<20} {stock:>5}  {reorder:>6}  {status}"
        stdscr.addstr(inventory_y + i + 1, 0, line[: width - 1])

        # Reset color after each line
        if curses.has_colors():
            stdscr.attroff(curses.A_COLOR)

    # Instructions
    instructions_y = inventory_y + max_items + 2
    if curses.has_colors():
        stdscr.attron(curses.color_pair(app.config.colors["accent"]))
    stdscr.addstr(instructions_y, 0, "↑/↓: Navigate  Enter: Adjust Stock  q: Quit")
    if curses.has_colors():
        stdscr.attroff(curses.color_pair(app.config.colors["accent"]))

    # Turn off default color
    if curses.has_colors():
        stdscr.attroff(curses.color_pair(app.config.colors["default"]))


def render_dialog(stdscr, app: App, height: int, width: int):
    """Render a dialog overlay"""
    if not app.show_dialog or not app.dialog_data:
        return

    # Dialog dimensions
    dialog_width = min(60, width - 4)
    dialog_height = min(15, height - 4)
    dialog_x = (width - dialog_width) // 2
    dialog_y = (height - dialog_height) // 2

    # Draw dialog box
    for y in range(dialog_y, dialog_y + dialog_height):
        for x in range(dialog_x, dialog_x + dialog_width):
            if y == dialog_y or y == dialog_y + dialog_height - 1:
                stdscr.addch(y, x, "-")
            elif x == dialog_x or x == dialog_x + dialog_width - 1:
                stdscr.addch(y, x, "|")
            else:
                stdscr.addch(y, x, " ")

    # Dialog title
    title = app.dialog_data.get("title", "Dialog")
    title_x = dialog_x + (dialog_width - len(title)) // 2
    stdscr.addstr(dialog_y, title_x, title)

    # Dialog content
    content = app.dialog_data.get("content", [])
    for i, line in enumerate(content):
        if i < dialog_height - 3:  # Leave space for title and border
            stdscr.addstr(dialog_y + i + 1, dialog_x + 2, line[: dialog_width - 4])


def render_footer(stdscr, app: App, height: int, width: int):
    """Render the footer with key hints"""
    footer_y = height - app.config.layout["footer_height"]

    # Footer with accent color
    if curses.has_colors():
        stdscr.attron(curses.color_pair(app.config.colors["accent"]))

    footer_text = "Press 'q' to quit, '?' for help"
    stdscr.addstr(footer_y, 0, footer_text)

    if curses.has_colors():
        stdscr.attroff(curses.color_pair(app.config.colors["accent"]))
