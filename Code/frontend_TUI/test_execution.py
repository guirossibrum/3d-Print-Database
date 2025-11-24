#!/usr/bin/env python3
"""
Test TUI execution without terminal check
"""

import sys
import os
import curses
import time

# Add the frontend_TUI directory to Python path
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from config import Config
from app import App


def test_execution(stdscr):
    """Test TUI execution"""
    print("Starting TUI execution test...")

    try:
        # Initialize curses
        curses.curs_set(0)
        stdscr.nodelay(True)
        stdscr.timeout(100)

        print("✓ Curses initialized")

        # Initialize colors
        if curses.has_colors():
            curses.start_color()
            curses.init_pair(1, curses.COLOR_WHITE, curses.COLOR_BLUE)
            curses.init_pair(2, curses.COLOR_WHITE, -1)
            curses.init_pair(3, curses.COLOR_WHITE, curses.COLOR_RED)
            curses.init_pair(4, curses.COLOR_BLACK, curses.COLOR_GREEN)
            curses.init_pair(5, curses.COLOR_BLACK, curses.COLOR_YELLOW)
            curses.init_pair(6, curses.COLOR_WHITE, -1)
            curses.init_pair(7, curses.COLOR_CYAN, -1)
            print("✓ Colors initialized")

        # Load configuration
        config = Config()
        print("✓ Config loaded")

        # Initialize application
        app = App(config)
        print("✓ App initialized")

        # Try to render something simple
        stdscr.erase()
        stdscr.addstr(0, 0, "TUI Test - Initialization successful!")
        stdscr.addstr(2, 0, "Press any key to exit...")
        stdscr.refresh()

        # Wait for a key press
        while True:
            key = stdscr.getch()
            if key != -1:
                break
            time.sleep(0.1)

        print("✓ TUI execution test completed successfully")
        return 0

    except Exception as e:
        print(f"✗ Error during TUI execution: {e}")
        import traceback

        traceback.print_exc()
        return 1


if __name__ == "__main__":
    try:
        exit_code = curses.wrapper(test_execution)
        sys.exit(exit_code)
    except Exception as e:
        print(f"✗ Curses wrapper error: {e}")
        import traceback

        traceback.print_exc()
        sys.exit(1)
