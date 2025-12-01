# frontend/modules/search.py
import requests
import tkinter as tk
from tkinter import messagebox
from .constants import SEARCH_URL


def search_products(
    search_query_entry,
    results_text_widget,
    search_results_list,
    include_inactive=False,
    include_prototype=False,
):
    """Search for products using unified search (empty query shows all products)"""
    # Get search query (allow empty for "show all")
    query = search_query_entry.get().strip()

    # Build query parameters (empty search_term parameter will show all products)
    params = {"search_term": query}

    try:
        response = requests.get(SEARCH_URL, params=params)
        if response.status_code == 200:
            search_results_list[:] = response.json()
            # Apply filters
            if not include_inactive:
                search_results_list[:] = [
                    p for p in search_results_list if p.get("active")
                ]
            if not include_prototype:
                search_results_list[:] = [
                    p for p in search_results_list if p.get("production")
                ]
            # Sort alphabetically by name
            search_results_list.sort(key=lambda x: x.get("name", "").lower())
            display_search_results(results_text_widget, search_results_list)
        else:
            results_text_widget.delete(1.0, tk.END)
            results_text_widget.insert(
                tk.END, f"Error: {response.status_code} - {response.text}"
            )
    except Exception as e:
        results_text_widget.delete(1.0, tk.END)
        results_text_widget.insert(tk.END, f"Error: {str(e)}")


def display_search_results(results_text_widget, search_results_list):
    """Display search results in the text widget"""
    results_text_widget.delete(1.0, tk.END)

    if not search_results_list:
        results_text_widget.insert(tk.END, "No products found.")
        return

    for i, product in enumerate(search_results_list):
        sku = str(product.get("sku", "N/A"))
        name = product.get("name", "N/A")
        description = product.get("description", "")
        # Handle tags as list of strings or dicts
        tags_list = product.get("tags", [])
        if tags_list and isinstance(tags_list[0], dict):
            tags = ", ".join(tag.get("name", "") for tag in tags_list)
        else:
            tags = ", ".join(tags_list)

        # Handle materials as list of strings or dicts
        materials_list = product.get("materials", [])
        if materials_list and isinstance(materials_list[0], dict):
            materials = ", ".join(mat.get("name", "") for mat in materials_list)
        else:
            materials = ", ".join(materials_list)

        # Handle rating
        rating = product.get("rating", 0)
        rating_display = " ".join("X" if i < rating else " " for i in range(5))

        production = "Production" if product.get("production") else "Prototype"
        active = "Active" if product.get("active") else "Inactive"

        results_text_widget.insert(tk.END, f"{i + 1}. {sku} - {name}\n")
        if description:
            results_text_widget.insert(tk.END, f"   Description: {description}\n")
        if tags:
            results_text_widget.insert(tk.END, f"   Tags: {tags}\n")
        if materials:
            results_text_widget.insert(tk.END, f"   Materials: {materials}\n")
        results_text_widget.insert(tk.END, f"   Rating: {rating_display}\n")
        results_text_widget.insert(tk.END, f"   Status: {production}\n")
        results_text_widget.insert(tk.END, f"   Active: {active}\n\n")


def load_product_from_search(
    results_text_widget, search_results_list, show_edit_callback
):
    """Load product from search results for editing (double-click)"""
    try:
        # Get cursor position
        cursor_pos = results_text_widget.index(tk.INSERT)
        line_num = int(cursor_pos.split(".")[0])

        # Extract the line content
        line_start = f"{line_num}.0"
        line_end = f"{line_num}.end"
        line_content = results_text_widget.get(line_start, line_end).strip()

        if not line_content:
            return

        # Parse the line to extract index number
        import re

        match = re.match(r"^(\d+)\.\s", line_content)
        if match:
            try:
                index_str = match.group(1)
                index = int(index_str) - 1  # Convert to 0-based index

                if 0 <= index < len(search_results_list):
                    product = search_results_list[index]
                    show_edit_callback(product)
                else:
                    messagebox.showwarning(
                        "Invalid Selection",
                        "Please double-click on a valid product line.",
                    )
            except (ValueError, IndexError):
                messagebox.showwarning(
                    "Invalid Selection",
                    "Please double-click on a product line with a number.",
                )
        else:
            # Check if we're on a line that belongs to a product (contains product data)
            # Look backwards to find the product header line
            current_line = line_num
            while current_line >= 1:
                line_start = f"{current_line}.0"
                line_end = f"{current_line}.end"
                line_content = results_text_widget.get(line_start, line_end).strip()
                match = re.match(r"^(\d+)\.\s", line_content)
                if match:
                    try:
                        index_str = match.group(1)
                        index = int(index_str) - 1
                        if 0 <= index < len(search_results_list):
                            product = search_results_list[index]
                            show_edit_callback(product)
                        return
                    except (ValueError, IndexError):
                        pass
                current_line -= 1

            messagebox.showwarning(
                "Invalid Selection",
                "Please double-click on a product line.",
            )
    except Exception as e:
        messagebox.showerror("Error", f"Error loading product: {str(e)}")

def update_available_tags(new_tags_list):
    """Update available tags list and refresh listboxes"""
    global all_available_tags
    for tag_name in new_tags_list:
        # Check if tag already exists (by name)
        existing = next((t for t in all_available_tags if t["name"] == tag_name), None)
        if not existing:
            # Add new tag with dummy ID
            all_available_tags.append({"id": None, "name": tag_name})
    all_available_tags.sort(key=lambda x: x["name"])
    # Update main listbox
    tag_listbox.delete(0, tk.END)
    for tag in all_available_tags:
        tag_listbox.insert(tk.END, tag["name"])
    # Update edit listbox if exists
    if "edit_tag_listbox" in globals():
        edit_tag_listbox.delete(0, tk.END)
        for tag in all_available_tags:
            edit_tag_listbox.insert(tk.END, tag["name"])

def filter_tag_list(event=None):
    """Filter the tag list based on input text"""
    filter_text = tag_entry.get().strip().lower()

    # Clear current list
    tag_listbox.delete(0, tk.END)

    # Filter and add matching tags
    for tag in all_available_tags:
        if not filter_text or filter_text in tag["name"].lower():
            tag_listbox.insert(tk.END, tag["name"])


def filter_material_list(event=None):
    """Filter the material list based on input text"""
    filter_text = material_entry.get().strip().lower()

    # Clear current list
    material_listbox.delete(0, tk.END)

    # Filter and add matching materials
    for material in all_available_materials:
        if not filter_text or filter_text in material["name"].lower():
            material_listbox.insert(tk.END, material["name"])