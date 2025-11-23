# frontend/modules/api_client.py
import requests
from .constants import API_URL, CATEGORIES_URL, TAGS_URL, SEARCH_URL


def save_product_changes(product_sku: str, payload: dict):
    """
    Save product changes via API.
    Returns True on success, raises Exception on failure.
    """
    response = requests.put(f"{API_URL}{product_sku}", json=payload)
    if response.status_code == 200:
        return True
    else:
        raise Exception(f"Failed to update product: {response.text}")


def apply_inventory_adjustment(
    sku: str, operation: str, quantity: int, current_stock: int
):
    """
    Apply inventory adjustment via API.
    Returns success message on success, raises Exception on failure.
    """
    if operation == "sold" and quantity > current_stock:
        raise ValueError(
            f"Cannot sell {quantity} items. Only {current_stock} in stock."
        )

    new_stock = (
        current_stock + quantity if operation == "printed" else current_stock - quantity
    )

    payload = {"stock_quantity": new_stock}
    response = requests.put(f"{API_URL}{sku}/inventory", json=payload)
    if response.status_code == 200:
        operation_text = "added to" if operation == "printed" else "removed from"
        return f"{quantity} items {operation_text} inventory for {sku}"
    else:
        raise Exception(f"Failed to update inventory: {response.text}")


def create_category_via_api(name: str, initials: str, description: str):
    """
    Create category via API.
    Returns the created category data on success, raises Exception on failure.
    """
    response = requests.post(
        CATEGORIES_URL,
        json={
            "name": name,
            "sku_initials": initials,
            "description": description,
        },
    )
    if response.status_code == 200:
        return response.json()
    else:
        raise Exception(f"Failed to create category: {response.text}")


def update_category_via_api(
    category_id: int, name: str, initials: str, description: str
):
    """
    Update category via API.
    Returns True on success, raises Exception on failure.
    """
    response = requests.put(
        f"{CATEGORIES_URL}/{category_id}",
        json={
            "name": name,
            "sku_initials": initials,
            "description": description,
        },
    )
    if response.status_code == 200:
        return True
    else:
        raise Exception(f"Failed to update category: {response.text}")
