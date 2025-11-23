# ensure_file_structure.py
import os
import json

BASE_DIR = os.path.join(os.path.dirname(__file__), "Products")

def create_product_folder(sku: str, name: str, description: str = "", tags=None, production: bool = False):
    """
    Creates folder structure and metadata.json for a product.
    """
    if tags is None:
        tags = []

    product_dir = os.path.join(BASE_DIR, sku)
    subfolders = ["images", "models", "notes", "print_files"]

    os.makedirs(product_dir, exist_ok=True)
    for sub in subfolders:
        os.makedirs(os.path.join(product_dir, sub), exist_ok=True)

    metadata = {
        "sku": sku,
        "name": name,
        "description": description,
        "tags": tags,
        "production": production   
    }

    metadata_file = os.path.join(product_dir, "metadata.json")
    with open(metadata_file, "w") as f:
        json.dump(metadata, f, indent=4)

    return product_dir, metadata_file


def update_metadata(sku: str, name=None, description=None, tags=None, production=None):
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

    with open(metadata_file, "w") as f:
        json.dump(data, f, indent=4)
