#!/bin/bash
set -e

echo "Building 3D Print Database TUI..."
cd "$(dirname "$(readlink -f "$0")")/Code/frontend"
cargo build --release

echo "Installing to ~/.local/bin/3D_Print_Database..."
cp target/release/printdb ~/.local/bin/3D_Print_Database

echo "Build complete! Binary installed at ~/.local/bin/3D_Print_Database"