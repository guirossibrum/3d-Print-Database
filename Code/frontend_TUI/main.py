#!/usr/bin/env python3
"""
3D Print Database TUI - Terminal User Interface
A modern TUI frontend for managing 3D printing products
"""

import sys
import os
import curses
import time
from typing import Optional

# Add the frontend_TUI directory to Python path
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from app import App
from config import Config
from handlers import handle_input
from ui import render_ui


def main(stdscr):
    """Main TUI application entry point"""
    # Initialize curses
    curses.curs_set(0)  # Hide cursor
    stdscr.nodelay(True)  # Non-blocking input
    stdscr.timeout(100)  # Refresh every 100ms
    stdscr.keypad(True)  # Enable keypad mode

    # Clear any existing content and set up the screen
    stdscr.clear()

    # Initialize colors if supported - use consistent background
    if curses.has_colors():
        curses.start_color()
        try:
            # Use black background for all pairs to ensure uniform appearance
            # This matches most terminal themes and avoids the -1 issues
            curses.init_pair(
                1, curses.COLOR_WHITE, curses.COLOR_BLACK
            )  # White on black
            curses.init_pair(
                2, curses.COLOR_CYAN, curses.COLOR_BLACK
            )  # Cyan on black (selected)
            curses.init_pair(
                3, curses.COLOR_RED, curses.COLOR_BLACK
            )  # Red on black (errors)
            curses.init_pair(
                4, curses.COLOR_GREEN, curses.COLOR_BLACK
            )  # Green on black (success)
            curses.init_pair(
                5, curses.COLOR_YELLOW, curses.COLOR_BLACK
            )  # Yellow on black (warnings)
            curses.init_pair(
                6, curses.COLOR_WHITE, curses.COLOR_BLACK
            )  # White on black (default)
            curses.init_pair(
                7, curses.COLOR_CYAN, curses.COLOR_BLACK
            )  # Cyan on black (accent)
        except curses.error:
            # If color initialization fails, continue without colors
            pass

    # Load configuration
    config = Config()

    # Initialize application
    try:
        app = App(config)
    except Exception as e:
        stdscr.addstr(0, 0, f"Failed to initialize application: {e}")
        stdscr.refresh()
        time.sleep(3)
        return 1

    # Main event loop
    while app.running:
        try:
            # Handle input
            key = stdscr.getch()
            if key != -1:  # -1 means no key pressed
                handle_input(app, key)

            # Render UI
            render_ui(stdscr, app)

            # Small delay to prevent excessive CPU usage
            time.sleep(0.01)

        except KeyboardInterrupt:
            break
        except Exception as e:
            # Display error and continue
            stdscr.clear()
            stdscr.addstr(0, 0, f"Error: {e}")
            stdscr.addstr(2, 0, "Press 'q' to quit")
            stdscr.refresh()

            # Wait for quit key
            while True:
                key = stdscr.getch()
                if key in [ord("q"), ord("Q")]:
                    break
                time.sleep(0.1)
            break

    return 0


if __name__ == "__main__":
    # Check if we're running in a terminal
    if not sys.stdout.isatty():
        print("This application must be run in a terminal", file=sys.stderr)
        sys.exit(1)

    # Run the TUI application
    exit_code = curses.wrapper(main)
    sys.exit(exit_code)
