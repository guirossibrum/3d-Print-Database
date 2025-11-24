#!/bin/bash
# Launch 3D Print Database TUI (Terminal User Interface)
# This script launches the terminal-based interface for managing 3D printing products

# Check if we're running in a terminal
if [ ! -t 0 ]; then
    echo "Error: TUI application must be run in a terminal."
    echo "Please run this from a terminal emulator."
    exit 1
fi

# Check if the Python script exists
SCRIPT_PATH="$HOME/Work/3d_print/Code/frontend_TUI/main.py"
if [ ! -f "$SCRIPT_PATH" ]; then
    echo "Error: TUI script not found at $SCRIPT_PATH"
    exit 1
fi

echo "Launching 3D Print Database TUI..."

# Change to the TUI directory and run the application
cd "$HOME/Work/3d_print/Code/frontend_TUI" || {
    echo "Error: Could not change to TUI directory"
    exit 1
}

# Launch the TUI application
python3 main.py

# Check exit code
EXIT_CODE=$?
if [ $EXIT_CODE -eq 0 ]; then
    echo "✓ 3D Print Database TUI exited successfully"
else
    echo "✗ 3D Print Database TUI exited with error code: $EXIT_CODE"
    echo "  This might be due to:"
    echo "  - Missing dependencies (run: pip install -r requirements.txt)"
    echo "  - Backend server not running (start with: cd Code/backend && uvicorn app.main:app --reload --host 0.0.0.0 --port 8000)"
    echo "  - Terminal compatibility issues"
    echo ""
    echo "  Try running directly: cd Code/frontend_TUI && python3 main.py"
    exit $EXIT_CODE
fi