#!/bin/bash
# Launch 3D Print Database GUI application
# This script detaches the GUI from the terminal so it runs independently

# Check if we have a display
if [ -z "$DISPLAY" ]; then
    echo "Error: No DISPLAY environment variable set. GUI cannot run without a graphical environment."
    echo "Make sure you're running this from a graphical desktop environment."
    echo "If you're using SSH, try: ssh -X username@hostname"
    exit 1
fi

# Check if the Python script exists
SCRIPT_PATH="$(pwd)/Code/frontend/3dPrintDB.py"
if [ ! -f "$SCRIPT_PATH" ]; then
    echo "Error: GUI script not found at $SCRIPT_PATH"
    exit 1
fi

echo "Launching 3D Print Database GUI..."

# Launch in background with proper detachment
setsid python3 "$SCRIPT_PATH" >/dev/null 2>&1 &
PID=$!

# Give it a moment to start
sleep 1

# Check if it's still running
if kill -0 $PID 2>/dev/null; then
    echo "✓ 3D Print Database GUI launched successfully (PID: $PID)"
    echo "  The application is now running in the background."
    echo "  You can close this terminal - the GUI will continue running."
else
    echo "✗ Failed to launch GUI application"
    echo "  This might be due to:"
    echo "  - Missing GUI libraries (tkinter)"
    echo "  - Display server issues"
    echo "  - Python environment problems"
    echo ""
    echo "  Try running directly: python3 "$(pwd)/Code/frontend/3dPrintDB.py""
    exit 1
fi
