# 3D Print Database

A comprehensive GUI application for managing 3D printing products with database integration and file system management.

## Features

- **Create Products**: Add new 3D printing products with basic information and automatic file structure generation
- **Search & Filter**: Find products by name, SKU, tags, or view all products with detailed match information
- **Edit Products**: Update existing product information and metadata with index-based selection
- **File Management**: Open product folders in file explorer and delete products (database only or database + files)
- **Category System**: Organize products with customizable categories and automatic SKU generation (CategoryInitials-001 format)
- **Tag Management**: Intelligent tag system with normalization, autocomplete suggestions, and usage statistics
- **Inventory Management**: Sales-focused inventory tracking with stock levels, reorder points, cost/pricing, and profit margin calculations

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
2. Choose a category (or create a new one) - determines SKU format (e.g., TOY-001)
3. Fill in basic product details:
   - Name (required)
   - Description
   - Production Ready checkbox
   - Tags (with autocomplete suggestions)
4. Click "Create Item" - this generates the file structure for design and printing

### Searching Products
1. Select the "Update Product" tab
2. Enter search terms in the search box (searches name, SKU, and tags)
3. Or leave empty to show all products
4. Click "Search"

### Editing Products
1. Search for products as above
2. **Double-click anywhere on a product line** (including details) to open the edit dialog
3. Modify product details including:
   - Basic info (name, description, production status, tags)
   - Manufacturing details (material, color, print time with smart formatting, weight)
4. Use the dialog buttons to:
   - **Save Changes**: Update the product
   - **Open Folder**: Browse product files
   - **Delete Record**: Remove the product
   - **Cancel**: Close dialog and re-enable double-click functionality

**Smart Time Formatting**: The print time field provides intelligent formatting with full editing freedom - type "0100" for instant "01:00" formatting, or edit any part freely with automatic completion when leaving the field

### Managing Files
- Click "Open Folder" to browse product files
- Click "Delete Record" to remove products (with confirmation)

### Inventory Management
1. Select the "Inventory" tab
2. Click "Refresh Inventory" to load current stock levels
3. **Quick Adjustments**: Double-click any product to open a simple dialog:
   - Enter quantity to adjust
   - Choose "Printed" (add to stock) or "Sold" (remove from stock)
   - Click "Apply" to update inventory
4. **Full Edit**: Click "Full Edit" to modify complete inventory settings
5. View profit margins and total inventory value in the summary

### Workflow
1. **Create Product**: Basic product information and automatic file structure generation
2. **Design & Print**: Use generated folders for CAD design and 3D printing
3. **Update Product**: Double-click search results to add manufacturing details (materials, times, weights)
4. **Set Pricing**: Configure costs and selling prices in Inventory → Full Edit
5. **Manage Daily Stock**: Use double-click adjustments for daily inventory changes (Printed/Sold)

### Inventory Management
1. Select the "Inventory" tab
2. Click "Refresh Inventory" to load current stock levels
3. Select a product and click "Update Stock" to modify:
   - Current stock quantity
   - Reorder point (minimum stock level)
   - Unit cost (production cost in cents)
   - Selling price (retail price in cents)
4. View profit margins and total inventory value in the summary

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
- **Backend**: FastAPI with SQLAlchemy, PostgreSQL, and MeiliSearch for full-text search
- **Frontend**: Tkinter GUI with streamlined workflow (Create → Design → Update → Inventory)
- **Database**: Category-based SKU generation with optional metadata and inventory fields
- **File System**: Automatic folder creation and cross-platform file management
- **Tag System**: Normalized tagging with autocomplete and search integration
- **Inventory System**: Sales-focused inventory management with quick adjustments and financial calculations

### Workflow Architecture
- **Create Tab**: Minimal required fields for initial product setup
- **Update Tab**: Manufacturing details and product metadata
- **Inventory Tab**: Stock management with double-click adjustments (Printed/Sold) and full editing