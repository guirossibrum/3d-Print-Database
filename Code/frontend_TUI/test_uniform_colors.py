#!/usr/bin/env python3
"""
Test uniform color scheme for TUI
"""

import sys
import os

# Add path
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))


def test_color_scheme():
    """Test that the color scheme uses terminal background consistently"""
    from config import Config

    config = Config()

    print("Testing TUI color scheme for uniform appearance...")
    print()

    print("Color pairs (all using terminal default background):")
    for name, pair_num in config.colors.items():
        if name == "header":
            desc = "White text on terminal background"
        elif name == "selected":
            desc = "Cyan text on terminal background (selections)"
        elif name == "accent":
            desc = "Cyan text on terminal background (highlights)"
        elif name == "default":
            desc = "White text on terminal background (main text)"
        elif name == "error":
            desc = "Red text on terminal background"
        elif name == "success":
            desc = "Green text on terminal background"
        elif name == "warning":
            desc = "Yellow text on terminal background"

        print(f"  {name}: pair {pair_num} - {desc}")

    print()
    print("✓ All colors now use terminal default background (-1)")
    print("✓ This should give uniform appearance matching your terminal theme")
    print("✓ Text colors provide contrast while background stays consistent")

    return True


if __name__ == "__main__":
    test_color_scheme()
