#!/usr/bin/env python3
"""
Quick TUI test to check color rendering
"""

import sys
import os
import curses
import time

# Add the frontend_TUI directory to Python path
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from config import Config
from app import App, Tab


def test_colors(stdscr):
    """Test color rendering in the TUI"""
    # Initialize curses
    curses.curs_set(0)
    stdscr.nodelay(True)
    stdscr.timeout(100)

    # Initialize colors
    if curses.has_colors():
        curses.start_color()
        curses.init_pair(1, curses.COLOR_WHITE, curses.COLOR_BLUE)  # Header
        curses.init_pair(2, curses.COLOR_WHITE, -1)  # Selected - white on default
        curses.init_pair(3, curses.COLOR_WHITE, curses.COLOR_RED)  # Error
        curses.init_pair(4, curses.COLOR_BLACK, curses.COLOR_GREEN)  # Success
        curses.init_pair(5, curses.COLOR_BLACK, curses.COLOR_YELLOW)  # Warning
        curses.init_pair(6, curses.COLOR_WHITE, -1)  # Default - white on default
        curses.init_pair(7, curses.COLOR_CYAN, -1)  # Accent - cyan on default

    # Clear screen
    stdscr.erase()

    # Test different color areas
    height, width = stdscr.getmaxyx()

    # Header (should be white on blue - working correctly)
    if curses.has_colors():
        stdscr.attron(curses.color_pair(1))
    stdscr.addstr(0, 0, "Header Test - White on Blue (should work)")
    if curses.has_colors():
        stdscr.attroff(curses.color_pair(1))

    # Selected item (now white on default background)
    if curses.has_colors():
        stdscr.attron(curses.color_pair(2))
    stdscr.addstr(2, 0, "Selected Test - White on Default Background")
    if curses.has_colors():
        stdscr.attroff(curses.color_pair(2))

    # Default text (white on default background)
    if curses.has_colors():
        stdscr.attron(curses.color_pair(6))
    stdscr.addstr(4, 0, "Default Text - White on Default Background")
    if curses.has_colors():
        stdscr.attroff(curses.color_pair(6))

    # Accent text (cyan on default background)
    if curses.has_colors():
        stdscr.attron(curses.color_pair(7))
    stdscr.addstr(6, 0, "Accent Text - Cyan on Default Background")
    if curses.has_colors():
        stdscr.attroff(curses.color_pair(7))

    # Instructions
    stdscr.addstr(8, 0, "Press 'q' to quit, 'c' to continue to full TUI")

    # Wait for user input
    while True:
        key = stdscr.getch()
        if key == ord("q"):
            return False
        elif key == ord("c"):
            return True
        time.sleep(0.1)


def main():
    """Main test function"""
    try:
        # Run color test first
        continue_to_full = curses.wrapper(test_colors)

        if continue_to_full:
            # Then run the full TUI
            from main import main as tui_main

            curses.wrapper(tui_main)

    except KeyboardInterrupt:
        pass
    except Exception as e:
        print(f"Error: {e}")


if __name__ == "__main__":
    main()
