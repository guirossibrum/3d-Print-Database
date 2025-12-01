# frontend/modules/api_client.py
import requests
from .constants import API_URL, TAGS_URL, MATERIALS_URL, CATEGORIES_URL


def api_request(method: str, url: str, data=None):
    """
    Generic API request helper.
    Handles common request/response logic.
    """
    try:
        response = requests.request(method, url, json=data)
        if response.status_code == 200:
            return response.json() if response.content else None
        else:
            raise Exception(f"API call failed: {response.text}")
    except Exception as e:
        raise Exception(f"API error: {str(e)}")


def save_product_changes(product_id: int, payload: dict):
    """
    Save product changes via API.
    Returns True on success, raises Exception on failure.
    """
    payload["product_id"] = product_id
    api_request("POST", f"{API_URL}", payload)
    return True


def create_tag(tag_name: str):
    """
    Create a new tag via API.
    Returns the created tag dict on success, raises Exception on failure.
    """
    payload = {"name": tag_name}
    return api_request("POST", f"{TAGS_URL}", payload)


def create_material(material_name: str):
    """
    Create a new material via API.
    Returns the created material dict on success, raises Exception on failure.
    """
    payload = {"name": material_name}
    return api_request("POST", f"{MATERIALS_URL}", payload)


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
    api_request("PUT", f"{API_URL}{sku}/inventory", payload)
    operation_text = "added to" if operation == "printed" else "removed from"
    return f"{quantity} items {operation_text} inventory for {sku}"


def create_category_via_api(name: str, initials: str, description: str):
    """
    Create category via API.
    Returns the created category data on success, raises Exception on failure.
    """
    data = {
        "name": name,
        "sku_initials": initials,
        "description": description,
    }
    return api_request("POST", CATEGORIES_URL, data)


def update_category_via_api(
    category_id: int, name: str, initials: str, description: str
):
    """
    Update category via API.
    Returns True on success, raises Exception on failure.
    """
    data = {
        "name": name,
        "sku_initials": initials,
        "description": description,
    }
    api_request("PUT", f"{CATEGORIES_URL}/{category_id}", data)
    return True
