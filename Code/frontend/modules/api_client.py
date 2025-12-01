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

def load_all_tags_for_list():
    """Load all existing tags to populate the listbox"""
    global all_available_tags

    try:
        response = requests.get(TAGS_URL, timeout=5)  # Synchronous for debugging
        if response.status_code == 200:
            data = response.json()
            all_available_tags = sorted(data, key=lambda x: x["name"])
            # Update listbox immediately
            filter_tag_list()
        else:
            # Show error for debugging
            ErrorDialog(
                root,
                "Tags Error",
                f"Failed to load tags: {response.status_code} - {response.text[:200]}",
            )
    except Exception as e:
        # Show error for debugging
        show_copyable_error("Tags Error", f"Error loading tags: {str(e)}")


def load_all_materials_for_list():
    """Load all existing materials to populate the listbox"""
    global all_available_materials

    try:
        response = requests.get("http://localhost:8000/materials", timeout=5)
        if response.status_code == 200:
            data = response.json()
            all_available_materials = sorted(data, key=lambda x: x["name"])
            # Update listboxes if exist
            if "edit_material_listbox" in globals():
                edit_material_listbox.delete(0, tk.END)
                for m in all_available_materials:
                    edit_material_listbox.insert(tk.END, m["name"])
            if "material_listbox" in globals():
                material_listbox.delete(0, tk.END)
                for m in all_available_materials:
                    material_listbox.insert(tk.END, m["name"])
        else:
            ErrorDialog(
                root,
                "Materials Error",
                f"Error loading materials: {response.status_code} - {response.text[:200]}",
            )
    except Exception as e:
        show_copyable_error("Materials Error", f"Error loading materials: {str(e)}")


def delete_unused_tag():
    """Delete the selected tag if it's not used by any products"""
    selection = tag_listbox.curselection()
    if not selection:
        messagebox.showwarning("No Selection", "Please select a tag to delete.")
        return

    selected_tag = tag_listbox.get(selection[0])

    # Confirm deletion
    if not messagebox.askyesno(
        "Confirm Deletion",
        f"Are you sure you want to delete the tag '{selected_tag}'?\n\n"
        "This will only work if the tag is not used by any products.",
    ):
        return

    try:
        # Check if tag is used and delete if unused
        response = requests.delete(f"{API_URL}../tags/{selected_tag}")
        if response.status_code == 200:
            # Refresh the tag list
            load_all_tags_for_list()
        elif response.status_code == 400:
            show_copyable_error(
                "Cannot Delete",
                f"Tag '{selected_tag}' is still used by products and cannot be deleted.",
            )
        else:
            show_copyable_error("Error", f"Failed to delete tag: {response.text}")
    except Exception as e:
        show_copyable_error("Error", f"Error deleting tag: {str(e)}")
        # No need to refresh list since we're using existing tags


def delete_unused_material():
    """Delete the selected material if it's not used by any products"""
    selection = material_listbox.curselection()
    if not selection:
        messagebox.showwarning("No Selection", "Please select a material to delete.")
        return

    selected_material = material_listbox.get(selection[0])

    # Confirm deletion
    if not messagebox.askyesno(
        "Confirm Deletion",
        f"Are you sure you want to delete the material '{selected_material}'?\n\n"
        "This will only work if the material is not used by any products.",
    ):
        return

    try:
        # Check if material is used and delete if unused
        response = requests.delete(
            f"http://localhost:8000/materials/{selected_material}"
        )
        if response.status_code == 200:
            # Refresh the material list
            load_all_materials_for_list()
        elif response.status_code == 400:
            show_copyable_error(
                "Cannot Delete",
                f"Material '{selected_material}' is still used by products and cannot be deleted.",
            )
        else:
            show_copyable_error("Error", f"Failed to delete material: {response.text}")
    except Exception as e:
        show_copyable_error("Error", f"Error deleting material: {str(e)}")
        # No need to refresh list since we're using existing materials

def load_categories():
    """Load categories from API"""
    global categories
    try:
        response = requests.get(CATEGORIES_URL)
        if response.status_code == 200:
            categories = response.json()
            update_category_dropdown()
        else:
            show_copyable_error("Error", f"Failed to load categories: {response.text}")
    except Exception as e:
        show_copyable_error("Error", f"Error loading categories: {str(e)}")

def load_inventory_status():
    """Load and display inventory status for all products"""
    global inventory_tree, include_out_of_stock_var, need_to_produce_var
    try:
        response = requests.get(INVENTORY_URL)
        if response.status_code == 200:
            inventory_data = response.json()

            # Filter data based on checkboxes
            filtered_data = []
            for item in inventory_data:
                if (
                    not include_out_of_stock_var.get()
                    and item.get("status") == "out_of_stock"
                    and item.get("reorder_point", 0) != 0
                ):
                    continue
                if need_to_produce_var.get() and item.get(
                    "stock_quantity", 0
                ) > item.get("reorder_point", 0):
                    continue
                filtered_data.append(item)

            # Clear existing items
            for item in inventory_tree.get_children():
                inventory_tree.delete(item)

            # Add inventory items
            total_value = 0
            low_stock_count = 0
            out_of_stock_count = 0

            for item in filtered_data:
                # Format values for display
                unit_cost = (
                    f"${item['unit_cost'] / 100:.2f}" if item["unit_cost"] else "N/A"
                )
                selling_price = (
                    f"${item['selling_price'] / 100:.2f}"
                    if item["selling_price"]
                    else "N/A"
                )
                total_value_item = (
                    f"${item['total_value'] / 100:.2f}"
                    if item["total_value"]
                    else "N/A"
                )
                profit_margin = (
                    f"{item['profit_margin']:.1f}%"
                    if item["profit_margin"] is not None
                    else "N/A"
                )

                # Color code status
                status = item["status"].replace("_", " ").title()
                if item["status"] == "out_of_stock":
                    status = "OUT OF STOCK"
                    out_of_stock_count += 1
                elif item["status"] == "low_stock":
                    status = "LOW STOCK"
                    low_stock_count += 1

                inventory_tree.insert(
                    "",
                    tk.END,
                    values=(
                        item["sku"],
                        item["name"],
                        item["stock_quantity"],
                        item["reorder_point"],
                        unit_cost,
                        selling_price,
                        total_value_item,
                        profit_margin,
                        status,
                    ),
                    tags=(item["id"],),
                )

                if item["total_value"]:
                    total_value += item["total_value"]

            # Update summary
            summary_text.config(state=tk.NORMAL)
            summary_text.delete(1.0, tk.END)
            summary_text.insert(
                tk.END,
                f"Total Products: {len(inventory_data)} | "
                f"Total Value: ${total_value / 100:.2f} | "
                f"Low Stock: {low_stock_count} | "
                f"Out of Stock: {out_of_stock_count}",
            )
            summary_text.config(state=tk.DISABLED)

        else:
            show_copyable_error("Error", f"Failed to load inventory: {response.text}")
    except Exception as e:
        show_copyable_error("Error", f"Error loading inventory: {str(e)}")

def sort_inventory_column(col):
    """Sort inventory Treeview by column"""
    global inventory_tree, inventory_sort_orders
    if col not in inventory_sort_orders:
        inventory_sort_orders[col] = True  # ascending first
    else:
        inventory_sort_orders[col] = not inventory_sort_orders[col]
    ascending = inventory_sort_orders[col]

    # Get all items with their values
    items = []
    for item in inventory_tree.get_children():
        values = inventory_tree.item(item, "values")
        items.append((values, item))

    # Define column index
    columns = (
        "sku",
        "name",
        "stock",
        "reorder",
        "cost",
        "price",
        "value",
        "margin",
        "status",
    )
    col_index = columns.index(col)

    def sort_key(item_values):
        val = item_values[0][col_index]
        if col in ("stock", "reorder", "cost", "price", "value"):
            try:
                return int(val) if val else 0
            except ValueError:
                return 0
        elif col == "margin":
            try:
                return float(val) if val else 0.0
            except ValueError:
                return 0.0
        else:
            return str(val).lower()

    items.sort(key=lambda x: sort_key(x), reverse=not ascending)

    # Clear tree
    for item in inventory_tree.get_children():
        inventory_tree.delete(item)

    # Reinsert sorted items
    for values, item_id in items:
        inventory_tree.insert("", "end", values=values)

def apply_inventory_adjustment(
    sku: str,
    product_id: int,
    operation,
    quantity: int,
    current_stock: int,
    reorder_point=None,
):
    """
    Apply inventory adjustment via API.
    Returns success message on success, raises Exception on failure.
    """
    if operation == "sold" and quantity > current_stock:
        raise ValueError(
            f"Cannot sell {quantity} items. Only {current_stock} in stock."
        )

    payload = {}
    if operation:
        new_stock = (
            current_stock + quantity
            if operation == "printed"
            else current_stock - quantity
        )
        payload["stock_quantity"] = new_stock

    if reorder_point is not None:
        payload["reorder_point"] = reorder_point

    if not payload:
        return "No changes made"

    response = requests.put(
        f"http://localhost:8000/inventory/{product_id}", json=payload
    )
    if response.status_code == 200:
        operation_text = "added to" if operation == "printed" else "removed from"
        msg = ""
        if operation:
            msg += f"{quantity} items {operation_text} inventory"
        if reorder_point is not None:
            if msg:
                msg += f" and reorder point set to {reorder_point}"
            else:
                msg += f"Reorder point set to {reorder_point}"
        return msg + f" for {sku}"
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


def save_product_changes(product_id: int, payload: dict):
    """
    Save product changes via API.
    Returns True on success, raises Exception on failure.
    """
    payload["product_id"] = product_id
    response = requests.post(API_URL, json=payload)
    if response.status_code == 200:
        return True
    else:
        raise Exception(f"Failed to update product: {response.text}")