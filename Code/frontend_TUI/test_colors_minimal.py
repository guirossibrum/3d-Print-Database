#!/usr/bin/env python3
"""
Minimal color test for TUI
"""

import curses
import os
import sys

# Add path
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))


def minimal_test():
    """Minimal test without full curses wrapper"""
    try:
        # Just test if we can import and basic setup works
        stdscr = curses.initscr()
        curses.curs_set(0)

        if curses.has_colors():
            curses.start_color()
            try:
                # Test the numeric color codes
                curses.init_pair(1, 7, 4)  # White on blue
                curses.init_pair(2, 0, 7)  # Black on white
                curses.init_pair(3, 7, 1)  # White on red
                curses.init_pair(4, 0, 2)  # Black on green
                curses.init_pair(5, 0, 3)  # Black on yellow
                curses.init_pair(6, 7, 0)  # White on black
                curses.init_pair(7, 6, 0)  # Cyan on black

                curses.endwin()
                return "SUCCESS: All color pairs initialized with numeric codes!"

            except curses.error as e:
                curses.endwin()
                return f"FAILED: Color init error: {e}"
        else:
            curses.endwin()
            return "No color support available"

    except Exception as e:
        try:
            curses.endwin()
        except:
            pass
        return f"Exception: {e}"


if __name__ == "__main__":
    result = minimal_test()
    print(result)
