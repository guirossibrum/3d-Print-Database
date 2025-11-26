#!/bin/bash
# Launch 3D Print Database TUI (Rust)
# This script launches the Rust-based terminal interface

# Check if we're running in a terminal
if [ ! -t 0 ]; then
    echo "Error: TUI application must be run in a terminal."
    echo "Please run this from a terminal emulator."
    exit 1
fi

# Check if the Rust binary exists in ~/.local/bin
RUST_BINARY="$HOME/.local/bin/3D_Print_Database"

if [ ! -f "$RUST_BINARY" ]; then
    echo "Error: Rust TUI binary not found at $RUST_BINARY"
    echo "Please run the install script first:"
    echo "  ./install_desktop_app_tui_rust.sh"
    exit 1
fi

echo "Using binary: $RUST_BINARY"

echo "Launching 3D Print Database TUI (Rust)..."

# Run the Rust TUI
"$RUST_BINARY"

# Check exit code
EXIT_CODE=$?
if [ $EXIT_CODE -eq 0 ]; then
    echo "✓ 3D Print Database TUI (Rust) exited successfully"
else
    echo "✗ 3D Print Database TUI (Rust) exited with error code: $EXIT_CODE"
    echo "  This might be due to:"
    echo "  - Backend server not running (start with: cd ../backend && uvicorn app.main:app --reload --host 0.0.0.0 --port 8000)"
    echo "  - Network connectivity issues"
    echo "  - Terminal compatibility issues"
    echo ""
    echo "  Try running directly: $RUST_BINARY"
    exit $EXIT_CODE
fi