# 3D Print Database TUI

A terminal-based user interface for managing 3D printing products, built with Python and curses.

## Features

- **Terminal-based Interface**: Clean, keyboard-driven TUI using curses
- **Product Management**: Create, search, edit, and delete 3D printing products
- **Inventory Tracking**: Monitor stock levels, reorder points, and profit margins
- **Category System**: Organize products with customizable categories
- **Tag System**: Intelligent tagging with autocomplete suggestions
- **Real-time Search**: Fast search across product names, SKUs, and tags

## Prerequisites

- Python 3.8+
- Running 3D Print Database backend (FastAPI server)
- Terminal with curses support

## Installation

1. Install dependencies:
```bash
cd Code/frontend_TUI
pip install -r requirements.txt
```

2. Ensure the backend is running:
```bash
cd Code/backend
uvicorn app.main:app --reload --host 0.0.0.0 --port 8000
```

## Usage

Run the TUI application:
```bash
cd Code/frontend_TUI
python main.py
```

## Key Bindings

### Global
- `Tab` / `Shift+Tab`: Switch between tabs
- `?`: Show help
- `q`: Quit application

### Navigation
- `j` / `↓`: Move down
- `k` / `↑`: Move up
- `h` / `←`: Move left
- `l` / `→`: Move right
- `Enter` / `Space`: Select item

### Create Tab
- `c`: Create product

### Search Tab
- `/`: Search products
- `e`: Edit selected product
- `d`: Delete selected product

### Inventory Tab
- `Enter`: Quick inventory adjustment

## Architecture

The TUI follows a modular architecture inspired by modern terminal applications:

```
frontend_TUI/
├── main.py              # Application entry point
├── app.py               # Central application state
├── config.py            # Configuration management
├── handlers/            # Input handling
│   └── __init__.py
├── services/            # Business logic & API communication
│   └── api_service.py
├── ui/                  # UI rendering components
│   └── __init__.py
└── utils/               # Utility functions
```

## Compatibility

This TUI frontend is fully compatible with the existing Tkinter GUI frontend and uses the same backend API endpoints. Both frontends can run simultaneously against the same backend server.

## Development

The TUI is built with:
- **curses**: For terminal UI rendering
- **requests**: For HTTP API communication
- **Modular design**: Easy to extend and maintain

## Limitations

- Currently read-only for most operations (search, view inventory)
- Product creation and editing interfaces are basic
- No advanced dialog systems yet
- Limited color support in some terminals

Future enhancements will include:
- Full CRUD operations with rich forms
- Advanced search and filtering
- Real-time inventory updates
- Better error handling and user feedback