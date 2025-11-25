#!/bin/bash
# Install 3D Print Database TUI (Rust) desktop application
# This script sets up the Rust TUI desktop file for easy launching

DESKTOP_FILE="$HOME/Work/3d_print/3D_Print_Database_TUI_RUST.desktop"
APPS_DIR="$HOME/.local/share/applications"

echo "Setting up 3D Print Database TUI (Rust) desktop application..."

# Check if Rust binary exists
RUST_BINARY="$HOME/Work/3d_print/frontend_tui_rust/target/debug/frontend_tui_rust"
if [ ! -f "$RUST_BINARY" ]; then
    echo "Warning: Rust binary not found. Building it now..."
    cd "$HOME/Work/3d_print/frontend_tui_rust" || {
        echo "Error: Could not find Rust TUI directory"
        exit 1
    }
    cargo build
    if [ $? -ne 0 ]; then
        echo "Error: Failed to build Rust TUI"
        exit 1
    fi
fi

# Create applications directory if it doesn't exist
mkdir -p "$APPS_DIR"

# Copy desktop file to applications directory
cp "$DESKTOP_FILE" "$APPS_DIR/"

# Make sure it's executable
chmod +x "$APPS_DIR/3D_Print_Database_TUI_RUST.desktop"

echo "✓ Rust TUI Desktop application installed!"
echo "You can now find '3D Print Database TUI (Rust)' in your applications menu"
echo "or double-click the desktop file to launch it."
echo ""
echo "To refresh your desktop environment, you may need to:"
echo "  - Log out and log back in"
echo "  - Run: update-desktop-database ~/.local/share/applications/"
echo "  - Or restart your desktop environment"
echo ""
echo "You can also launch it directly with: ./3dPrintDB_TUI_RUST.sh"