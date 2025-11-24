"""
Application state management for the 3D Print Database TUI
"""

import time
import threading
from typing import List, Dict, Any, Optional
from enum import Enum

from config import Config
from services.api_service import APIService


class Tab(Enum):
    """Main application tabs"""

    CREATE = "create"
    SEARCH = "search"
    INVENTORY = "inventory"


class Focus(Enum):
    """UI focus states"""

    TABS = "tabs"
    CONTENT = "content"
    DIALOG = "dialog"


class App:
    """Main application state"""

    def __init__(self, config: Config):
        self.config = config
        self.running = True

        # UI state
        self.current_tab = Tab.SEARCH
        self.focus = Focus.TABS
        self.selected_tab_index = 0
        self.tabs = [Tab.CREATE, Tab.SEARCH, Tab.INVENTORY]

        # Data state
        self.products: List[Dict[str, Any]] = []
        self.categories: List[Dict[str, Any]] = []
        self.tags: List[str] = []
        self.inventory: List[Dict[str, Any]] = []

        # Search state
        self.search_query = ""
        self.search_results: List[Dict[str, Any]] = []
        self.selected_product_index = 0

        # Create form state
        self.create_form = {
            "name": "",
            "description": "",
            "category_id": None,
            "tags": [],
            "production": True,
        }

        # Dialog state
        self.show_dialog = False
        self.dialog_type = None
        self.dialog_data = {}

        # Status and messages
        self.status_message = ""
        self.status_type = "info"  # info, success, error, warning
        self.last_refresh = 0

        # Services
        self.api = APIService(config)

        # Background refresh thread
        self.refresh_thread = None
        self.stop_refresh = False

        # Initialize data
        self._load_initial_data()

    def _load_initial_data(self):
        """Load initial data from API"""
        try:
            # Load categories
            self.categories = self.api.get_categories()

            # Load tags
            tags_data = self.api.get_tags()
            self.tags = [tag["name"] for tag in tags_data]

            # Load initial products (empty search)
            self.search_results = self.api.search_products("")
            self.products = self.search_results.copy()

            # Load inventory
            self.inventory = self.api.get_inventory_status()

            self.status_message = "Data loaded successfully"
            self.status_type = "success"

        except Exception as e:
            self.status_message = f"Failed to load data: {e}"
            self.status_type = "error"

    def switch_tab(self, tab: Tab):
        """Switch to a different tab"""
        self.current_tab = tab
        self.selected_tab_index = self.tabs.index(tab)
        self.focus = Focus.CONTENT

        # Refresh data for the new tab
        if tab == Tab.SEARCH:
            self._refresh_search_results()
        elif tab == Tab.INVENTORY:
            self._refresh_inventory()

    def next_tab(self):
        """Switch to next tab"""
        self.selected_tab_index = (self.selected_tab_index + 1) % len(self.tabs)
        self.switch_tab(self.tabs[self.selected_tab_index])

    def prev_tab(self):
        """Switch to previous tab"""
        self.selected_tab_index = (self.selected_tab_index - 1) % len(self.tabs)
        self.switch_tab(self.tabs[self.selected_tab_index])

    def search_products(self, query: str):
        """Search for products"""
        self.search_query = query
        try:
            self.search_results = self.api.search_products(query)
            self.selected_product_index = 0
            self.status_message = f"Found {len(self.search_results)} products"
            self.status_type = "info"
        except Exception as e:
            self.status_message = f"Search failed: {e}"
            self.status_type = "error"

    def _refresh_search_results(self):
        """Refresh search results"""
        self.search_products(self.search_query)

    def _refresh_inventory(self):
        """Refresh inventory data"""
        try:
            self.inventory = self.api.get_inventory_status()
            self.status_message = "Inventory refreshed"
            self.status_type = "success"
        except Exception as e:
            self.status_message = f"Failed to refresh inventory: {e}"
            self.status_type = "error"

    def select_product(self, index: int):
        """Select a product from search results"""
        if 0 <= index < len(self.search_results):
            self.selected_product_index = index

    def get_selected_product(self) -> Optional[Dict[str, Any]]:
        """Get currently selected product"""
        if self.search_results and 0 <= self.selected_product_index < len(
            self.search_results
        ):
            return self.search_results[self.selected_product_index]
        return None

    def create_product(self) -> bool:
        """Create a new product"""
        try:
            # Validate form data
            if not self.create_form["name"].strip():
                self.status_message = "Product name is required"
                self.status_type = "error"
                return False

            if not self.create_form["category_id"]:
                self.status_message = "Category selection is required"
                self.status_type = "error"
                return False

            # Create product via API
            result = self.api.create_product(self.create_form)

            if result:
                self.status_message = f"Product created: {result.get('sku', 'Unknown')}"
                self.status_type = "success"

                # Clear form
                self.create_form = {
                    "name": "",
                    "description": "",
                    "category_id": None,
                    "tags": [],
                    "production": True,
                }

                # Refresh data
                self._refresh_search_results()
                return True
            else:
                self.status_message = "Failed to create product"
                self.status_type = "error"
                return False

        except Exception as e:
            self.status_message = f"Error creating product: {e}"
            self.status_type = "error"
            return False

    def update_product(self, sku: str, updates: Dict[str, Any]) -> bool:
        """Update an existing product"""
        try:
            result = self.api.update_product(sku, updates)
            if result:
                self.status_message = f"Product {sku} updated successfully"
                self.status_type = "success"
                self._refresh_search_results()
                return True
            else:
                self.status_message = f"Failed to update product {sku}"
                self.status_type = "error"
                return False
        except Exception as e:
            self.status_message = f"Error updating product: {e}"
            self.status_type = "error"
            return False

    def delete_product(self, sku: str) -> bool:
        """Delete a product"""
        try:
            result = self.api.delete_product(sku)
            if result:
                self.status_message = f"Product {sku} deleted successfully"
                self.status_type = "success"
                self._refresh_search_results()
                return True
            else:
                self.status_message = f"Failed to delete product {sku}"
                self.status_type = "error"
                return False
        except Exception as e:
            self.status_message = f"Error deleting product: {e}"
            self.status_type = "error"
            return False

    def update_inventory(
        self, sku: str, stock_quantity: int, reorder_point: int = None
    ) -> bool:
        """Update inventory for a product"""
        try:
            updates = {"stock_quantity": stock_quantity}
            if reorder_point is not None:
                updates["reorder_point"] = reorder_point

            result = self.api.update_inventory(sku, updates)
            if result:
                self.status_message = f"Inventory updated for {sku}"
                self.status_type = "success"
                self._refresh_inventory()
                return True
            else:
                self.status_message = f"Failed to update inventory for {sku}"
                self.status_type = "error"
                return False
        except Exception as e:
            self.status_message = f"Error updating inventory: {e}"
            self.status_type = "error"
            return False

    def show_help(self):
        """Show help dialog"""
        self.show_dialog = True
        self.dialog_type = "help"
        self.dialog_data = {
            "title": "Help",
            "content": [
                "Navigation:",
                "  Tab/Shift+Tab: Switch tabs",
                "  j/k or ↑/↓: Navigate lists",
                "  Enter/Space: Select item",
                "  /: Search products",
                "  c: Create product (Create tab)",
                "  e: Edit selected product",
                "  d: Delete selected product",
                "  q: Quit application",
                "  ?: Show this help",
                "",
                "Tabs:",
                "  Create: Add new products",
                "  Search: Find and edit products",
                "  Inventory: Manage stock levels",
            ],
        }

    def hide_dialog(self):
        """Hide current dialog"""
        self.show_dialog = False
        self.dialog_type = None
        self.dialog_data = {}

    def quit(self):
        """Quit the application"""
        self.running = False
        self.stop_refresh = True

    def tick(self):
        """Periodic update tick"""
        current_time = time.time()

        # Auto-refresh if enabled
        if (
            self.config.settings["auto_refresh"]
            and current_time - self.last_refresh
            > self.config.settings["refresh_interval"]
        ):
            self._refresh_search_results()
            self._refresh_inventory()
            self.last_refresh = current_time
