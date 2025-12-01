# ensure_file_structure.py
import os
import json

# Get products directory from environment variable or default to user's Work directory
BASE_DIR = os.environ.get(
    "PRODUCTS_DIR",
    os.path.join(os.path.expanduser("~"), "Work", "3d_print", "Products"),
)


def create_product_folder(
    sku: str,
    name: str,
    description: str = "",
    tags=None,
    production: bool = False,
    active: bool = True,
    materials=None,
    category=None,
    rating: int = 0,
):
    """Create folder structure and metadata.json for a product."""
    if tags is None:
        tags = []
    if materials is None:
        materials = []

    product_dir = os.path.join(BASE_DIR, f"{sku} - {name}")
    subfolders = ["images", "models", "notes", "print_files"]

    print(f"DEBUG: BASE_DIR = {BASE_DIR}")
    print(f"DEBUG: product_dir = {product_dir}")
    print(f"DEBUG: cwd = {os.getcwd()}")

    os.makedirs(product_dir, exist_ok=True)
    for sub in subfolders:
        os.makedirs(os.path.join(product_dir, sub), exist_ok=True)

    metadata = {
        "sku": sku,
        "name": name,
        "description": description,
        "category": category,
        "tags": tags,
        "materials": materials,
        "production": production,
        "active": active,
        "color": None,
        "print_time": None,
        "weight": None,
        "rating": rating,
        "stock_quantity": 0,
        "reorder_point": 0,
        "unit_cost": None,
        "selling_price": None,
    }

    metadata_file = os.path.join(product_dir, "metadata.json")
    with open(metadata_file, "w") as f:
        json.dump(metadata, f, indent=4)

    print(f"DEBUG: Successfully created {product_dir}")
    return product_dir, metadata_file
