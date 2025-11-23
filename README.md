# 3D Print Database

A comprehensive GUI application for managing 3D printing products with database integration and file system management.

## Features

- **Create Products**: Add new 3D printing products with categories, tags, and descriptions
- **Search & Filter**: Find products by name, SKU, tags, or view all products
- **Edit Products**: Update existing product information and metadata
- **File Management**: Open product folders and manage associated files
- **Category System**: Organize products with customizable categories and SKUs

## Installation

### Option 1: Desktop Application (Recommended)
```bash
# Install the desktop application
./install_desktop_app.sh

# The application will appear in your applications menu
# You can also double-click the 3D_Print_Database.desktop file
```

### Option 2: Command Line
```bash
# Start the backend services
docker-compose -f Code/infra/docker-compose.yml up -d

# Launch the GUI
./3dPrintDB.sh
```

## Usage

### Creating Products
1. Select the "Create Product" tab
2. Choose a category (or create a new one)
3. Fill in product details (name, description, tags)
4. Click "Create Item"

### Searching Products
1. Select the "Update Product" tab
2. Enter search terms in the search box (searches name, SKU, and tags)
3. Or leave empty to show all products
4. Click "Search"

### Editing Products
1. Search for products as above
2. Enter the index number of the product you want to edit
3. Click "Load for Edit"
4. Modify the product details
5. Click "Update Product"

### Managing Files
- Click "Open Folder" to browse product files
- Click "Delete Record" to remove products (with confirmation)

## Requirements

- Python 3.8+
- Docker and Docker Compose
- Graphical desktop environment
- PostgreSQL (provided via Docker)
- MeiliSearch (provided via Docker)

## Project Structure

```
Work/3d_print/
├── Code/                    # Source code
│   ├── backend/            # FastAPI server
│   ├── frontend/           # GUI application
│   └── infra/              # Docker configuration
├── Products/               # Product files and folders
├── 3dPrintDB.sh           # Launch script
├── 3D_Print_Database.desktop  # Desktop application
└── install_desktop_app.sh # Desktop app installer
```

## Troubleshooting

### Application won't start
- Ensure Docker services are running: `docker-compose -f Code/infra/docker-compose.yml ps`
- Check DISPLAY environment: `echo $DISPLAY`
- Verify Python dependencies

### Desktop app doesn't appear
- Run: `./install_desktop_app.sh`
- Log out and log back in
- Check: `ls ~/.local/share/applications/`

### Permission issues
- Run: `./Code/fix_permissions.sh`
- Ensure Docker containers have proper access

## Development

The application consists of:
- **Backend**: FastAPI with SQLAlchemy and PostgreSQL
- **Frontend**: Tkinter GUI with tabbed interface
- **Database**: Category-based SKU generation
- **File System**: Automatic folder creation and management