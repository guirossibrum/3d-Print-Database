#!/bin/bash
# Launch 3D Print Database TUI (Rust)
# This script launches the Rust-based terminal interface

# Check if we're running in a terminal
if [ ! -t 0 ]; then
    echo "Error: TUI application must be run in a terminal."
    echo "Please run this from a terminal emulator."
    exit 1
fi

# Check if the Rust binary exists (prefer release, fallback to debug)
RUST_BINARY_RELEASE="$HOME/Work/3d_print/frontend_tui_rust/target/release/frontend_tui_rust"
RUST_BINARY_DEBUG="$HOME/Work/3d_print/frontend_tui_rust/target/debug/frontend_tui_rust"

if [ -f "$RUST_BINARY_RELEASE" ]; then
    RUST_BINARY="$RUST_BINARY_RELEASE"
    BINARY_TYPE="release"
elif [ -f "$RUST_BINARY_DEBUG" ]; then
    RUST_BINARY="$RUST_BINARY_DEBUG"
    BINARY_TYPE="debug"
else
    echo "Error: Rust TUI binary not found"
    echo "Please build it first with:"
    echo "  cd frontend_tui_rust && cargo build --release  (recommended)"
    echo "  or: cd frontend_tui_rust && cargo build        (debug)"
    exit 1
fi

echo "Using $BINARY_TYPE binary: $RUST_BINARY"

echo "Launching 3D Print Database TUI (Rust)..."

# Change to the project directory and run the binary
cd "$HOME/Work/3d_print/frontend_tui_rust" || {
    echo "Error: Could not change to TUI directory"
    exit 1
}

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