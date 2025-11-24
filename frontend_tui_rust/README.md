# 3D Print Database TUI (Rust)

A modern terminal user interface for managing 3D printing products, built with Rust and ratatui.

## Features

- **Terminal-based Interface**: Clean, keyboard-driven TUI using ratatui
- **Product Management**: Create, search, edit, and delete 3D printing products
- **Category & Tag Support**: Full support for categories and tags
- **Real-time Updates**: Live data from the FastAPI backend
- **Vim-like Navigation**: j/k for navigation, Tab for switching tabs
- **Uniform Colors**: Consistent color scheme that works with terminal themes

## Prerequisites

- **Rust**: 1.70+ (install from [rustup.rs](https://rustup.rs/))
- **Backend**: Running 3D Print Database FastAPI server
- **Terminal**: Modern terminal with UTF-8 support

## Installation

1. **Install Rust** (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

2. **Build the TUI**:
   ```bash
   cd frontend_tui_rust
   cargo build --release
   ```

3. **Install desktop integration** (optional):
   ```bash
   cd ..
   ./install_desktop_app_tui_rust.sh
   ```

## Usage

### Direct Launch
```bash
cd frontend_tui_rust
cargo run
# Or with the built binary:
./target/release/frontend_tui_rust
```

### Via Launcher Script
```bash
./3dPrintDB_TUI_RUST.sh
```

### Desktop Integration
- Install with: `./install_desktop_app_tui_rust.sh`
- Look for "3D Print Database TUI (Rust)" in your applications menu

## Key Bindings

### Global
- `Tab` / `Shift+Tab`: Switch between tabs (Create/Search/Inventory)
- `q` / `Ctrl+c`: Quit application
- `?`: Show help (not implemented yet)

### Navigation
- `j` / `↓`: Move down in lists
- `k` / `↑`: Move up in lists
- `Enter`: Select current item

### Actions
- `Ctrl+d`: Delete selected item (with confirmation)

## Architecture

```
frontend_tui_rust/
├── src/
│   ├── main.rs              # Application entry point
│   ├── app.rs               # Application state & logic
│   ├── ui.rs                # TUI rendering with ratatui
│   └── api.rs               # HTTP client for backend API
├── Cargo.toml               # Dependencies
└── README.md               # This file
```

### Key Components

- **ratatui**: Terminal UI framework (same as impala)
- **crossterm**: Cross-platform terminal manipulation
- **reqwest**: HTTP client for API communication
- **tokio**: Async runtime for network operations
- **serde**: JSON serialization/deserialization

## Backend Compatibility

This Rust TUI uses the **exact same API endpoints** as the Python frontends:

- `GET /categories` - Load categories
- `GET /tags` - Load tags
- `GET /products/search?q={query}` - Search products
- `POST /products/` - Create products
- `PUT /products/{sku}` - Update products
- `DELETE /products/{sku}` - Delete products

## Color Scheme

Unlike the Python curses version, this Rust TUI uses ratatui's color system which provides:

- **Consistent rendering** across different terminals
- **Better theme compatibility** with modern terminals
- **No color flickering** or background issues
- **Proper Unicode support** for icons and symbols

## Development

### Building
```bash
cargo build          # Debug build
cargo build --release # Optimized release build
```

### Running
```bash
cargo run           # Run with debug build
cargo run --release # Run with release build
```

### Testing
```bash
cargo test          # Run unit tests
cargo check         # Check compilation without building
```

## Comparison with Python TUI

| Feature | Python (curses) | Rust (ratatui) |
|---------|----------------|----------------|
| **Color Handling** | Problematic | Excellent |
| **Terminal Compatibility** | Limited | Broad |
| **Performance** | Good | Excellent |
| **Memory Usage** | Higher | Lower |
| **Build Size** | N/A (interpreted) | Small binary |
| **Dependencies** | Python + curses | Standalone binary |

## Troubleshooting

### Colors not displaying correctly
- Ensure your terminal supports 256 colors or true color
- Try a different terminal emulator (Ghostty, Alacritty, etc.)

### Backend connection issues
- Verify the FastAPI server is running: `http://localhost:8000`
- Check network connectivity
- Review API endpoint compatibility

### Compilation issues
- Ensure Rust 1.70+ is installed
- Update dependencies: `cargo update`
- Clean and rebuild: `cargo clean && cargo build`

## Future Enhancements

- [ ] Advanced search and filtering
- [ ] Product editing interface
- [ ] Inventory management features
- [ ] Keyboard shortcuts help screen
- [ ] Configuration file support
- [ ] Theme customization
- [ ] Export/import functionality

## License

Same as the main project.