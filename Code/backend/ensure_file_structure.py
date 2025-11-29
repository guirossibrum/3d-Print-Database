# ensure_file_structure.py
import os
import json

# Get products directory from environment variable or default to user's Work directory
BASE_DIR = os.environ.get(
    "PRODUCTS_DIR",
    os.path.join(os.path.expanduser("~"), "Work", "3d_print", "Products"),
)


def create_product_folder_debug(sku: str):
    """Debug version - minimal folder creation only"""
    product_dir = os.path.join(BASE_DIR, sku)
    print(f"DEBUG: BASE_DIR = {BASE_DIR}")
    print(f"DEBUG: product_dir = {product_dir}")
    print(f"DEBUG: cwd = {os.getcwd()}")

    os.makedirs(product_dir, exist_ok=True)
    print(f"DEBUG: Successfully created {product_dir}")
    return product_dir


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
        data["materials"] = material
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
