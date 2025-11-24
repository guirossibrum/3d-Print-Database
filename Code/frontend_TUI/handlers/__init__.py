"""
Input handling for the 3D Print Database TUI
"""

from app import App, Tab, Focus


def handle_input(app: App, key: int):
    """Main input handler - dispatch to appropriate handler based on context"""

    # Global key bindings that work in any context
    if key in app.config.key_bindings["quit"]:
        app.quit()
        return

    if key in app.config.key_bindings["help"]:
        app.show_help()
        return

    # Handle dialog input first if dialog is shown
    if app.show_dialog:
        handle_dialog_input(app, key)
        return

    # Handle tab switching
    if key in app.config.key_bindings["tab_next"]:
        app.next_tab()
        return
    elif key in app.config.key_bindings["tab_prev"]:
        app.prev_tab()
        return

    # Handle tab-specific input
    if app.current_tab == Tab.CREATE:
        handle_create_tab_input(app, key)
    elif app.current_tab == Tab.SEARCH:
        handle_search_tab_input(app, key)
    elif app.current_tab == Tab.INVENTORY:
        handle_inventory_tab_input(app, key)


def handle_dialog_input(app: App, key: int):
    """Handle input when a dialog is shown"""
    if key in [27, ord("q"), ord("Q")]:  # ESC, q, Q
        app.hide_dialog()
    elif key in app.config.key_bindings["select"]:
        # Handle dialog-specific actions
        if app.dialog_type == "help":
            app.hide_dialog()


def handle_create_tab_input(app: App, key: int):
    """Handle input for the create tab"""
    if key in app.config.key_bindings["create"]:
        app.create_product()


def handle_search_tab_input(app: App, key: int):
    """Handle input for the search tab"""
    if key in app.config.key_bindings["search"]:
        # Enter search mode - for now just clear search
        app.search_products("")
    elif key in app.config.key_bindings["up"]:
        # Navigate up in search results
        if app.selected_product_index > 0:
            app.selected_product_index -= 1
    elif key in app.config.key_bindings["down"]:
        # Navigate down in search results
        if app.selected_product_index < len(app.search_results) - 1:
            app.selected_product_index += 1
    elif key in app.config.key_bindings["select"]:
        # Select current product for editing
        product = app.get_selected_product()
        if product:
            # For now, just show a message
            app.status_message = f"Selected: {product['name']} ({product['sku']})"
            app.status_type = "info"
    elif key in app.config.key_bindings["edit"]:
        # Edit selected product
        product = app.get_selected_product()
        if product:
            app.status_message = f"Edit mode for: {product['name']}"
            app.status_type = "info"
    elif key in app.config.key_bindings["delete"]:
        # Delete selected product
        product = app.get_selected_product()
        if product:
            if app.delete_product(product["sku"]):
                app.status_message = f"Deleted: {product['name']}"
                app.status_type = "success"


def handle_inventory_tab_input(app: App, key: int):
    """Handle input for the inventory tab"""
    # Similar navigation logic as search tab
    if key in app.config.key_bindings["up"]:
        if app.selected_product_index > 0:
            app.selected_product_index -= 1
    elif key in app.config.key_bindings["down"]:
        if app.selected_product_index < len(app.inventory) - 1:
            app.selected_product_index += 1
    elif key in app.config.key_bindings["select"]:
        # Quick inventory adjustment
        if app.inventory and 0 <= app.selected_product_index < len(app.inventory):
            product = app.inventory[app.selected_product_index]
            app.status_message = f"Quick adjust inventory for: {product['name']}"
            app.status_type = "info"
