"""
Configuration for the 3D Print Database TUI
"""

import os
from typing import Dict, Any


class Config:
    """Application configuration"""

    def __init__(self):
        # API endpoints
        self.api_base_url = os.getenv("API_BASE_URL", "http://localhost:8000")

        # API endpoints
        self.api_urls = {
            "products": f"{self.api_base_url}/products/",
            "search": f"{self.api_base_url}/products/search",
            "tags": f"{self.api_base_url}/tags",
            "tag_suggest": f"{self.api_base_url}/tags/suggest",
            "categories": f"{self.api_base_url}/categories",
            "inventory": f"{self.api_base_url}/inventory/status",
        }

        # UI settings - using black background for uniform appearance
        self.colors = {
            "header": 1,  # White text on black background
            "selected": 2,  # Cyan text on black background (for selections)
            "error": 3,  # Red text on black background
            "success": 4,  # Green text on black background
            "warning": 5,  # Yellow text on black background
            "default": 6,  # White text on black background (main text)
            "accent": 7,  # Cyan text on black background (highlights)
        }

        # Key bindings
        self.key_bindings = {
            "quit": [ord("q"), ord("Q")],
            "tab_next": [9],  # Tab
            "tab_prev": [],  # Shift+Tab not easily handled in curses
            "select": [ord("\n"), ord(" ")],  # Enter, Space
            "up": [ord("k"), curses.KEY_UP],
            "down": [ord("j"), curses.KEY_DOWN],
            "left": [ord("h"), curses.KEY_LEFT],
            "right": [ord("l"), curses.KEY_RIGHT],
            "search": [ord("/")],
            "create": [ord("c")],
            "edit": [ord("e")],
            "delete": [ord("d")],
            "help": [ord("?")],
        }

        # UI layout
        self.layout = {
            "header_height": 3,
            "footer_height": 2,
            "sidebar_width": 25,
            "main_margin": 2,
        }

        # Application settings
        self.settings = {
            "auto_refresh": True,
            "refresh_interval": 30,  # seconds
            "max_search_results": 50,
            "confirm_deletions": True,
        }


# Import curses here to avoid circular imports
import curses
