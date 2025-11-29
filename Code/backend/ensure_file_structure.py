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
    materials=None,
    category=None,
):
    """
    Creates folder structure and metadata.json for a product.
    """
    if tags is None:
        tags = []
    if materials is None:
        materials = []

    product_dir = os.path.join(BASE_DIR, sku)
    subfolders = ["images", "models", "notes", "print_files"]

    os.makedirs(product_dir, exist_ok=True)
    for sub in subfolders:
        os.makedirs(os.path.join(product_dir, sub), exist_ok=True)

    metadata = {
        "sku": sku,
        "name": name,
        "description": description,
        "category": category,  # Add category field
        "tags": tags,
        "materials": materials,  # Store materials array from start
        "production": production,
        "color": None,
        "print_time": None,
        "weight": None,
        "stock_quantity": 0,
        "reorder_point": 0,
        "unit_cost": None,
        "selling_price": None,
    }

    metadata_file = os.path.join(product_dir, "metadata.json")
    with open(metadata_file, "w") as f:
        json.dump(metadata, f, indent=4)

    return product_dir, metadata_file


def update_metadata(
    sku: str,
    name=None,
    description=None,
    tags=None,
    production=None,
    material=None,
    color=None,
    print_time=None,
    weight=None,
    stock_quantity=None,
    reorder_point=None,
    unit_cost=None,
    selling_price=None,
):
    """
    Updates metadata.json based on DB changes.
    """
    product_dir = os.path.join(BASE_DIR, sku)
    metadata_file = os.path.join(product_dir, "metadata.json")
    if not os.path.exists(metadata_file):
        raise FileNotFoundError(f"Product folder for SKU {sku} does not exist.")

    with open(metadata_file, "r") as f:
        data = json.load(f)

    if name is not None:
        data["name"] = name
    if description is not None:
        data["description"] = description
    if tags is not None:
        data["tags"] = tags
    if production is not None:
        data["production"] = production
    if material is not None:
        data["material"] = material
    if color is not None:
        data["color"] = color
    if print_time is not None:
        data["print_time"] = print_time
    if weight is not None:
        data["weight"] = weight
    if stock_quantity is not None:
        data["stock_quantity"] = stock_quantity
    if reorder_point is not None:
        data["reorder_point"] = reorder_point
    if unit_cost is not None:
        data["unit_cost"] = unit_cost
    if selling_price is not None:
        data["selling_price"] = selling_price

    with open(metadata_file, "w") as f:
        json.dump(data, f, indent=4)
