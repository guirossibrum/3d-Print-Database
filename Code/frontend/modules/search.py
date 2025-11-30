# frontend/modules/search.py
import requests
import tkinter as tk
from tkinter import messagebox
from .constants import SEARCH_URL


def search_products(search_query_entry, results_text_widget, search_results_list):
    """Search for products using unified search (empty query shows all products)"""
    # Get search query (allow empty for "show all")
    query = search_query_entry.get().strip()

    # Build query parameters (empty search_term parameter will show all products)
    params = {"search_term": query}

    try:
        response = requests.get(SEARCH_URL, params=params)
        if response.status_code == 200:
            search_results_list[:] = response.json()
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
        production = "Production" if product.get("production") else "Prototype"

        results_text_widget.insert(tk.END, f"{i + 1}. {sku} - {name}\n")
        if description:
            results_text_widget.insert(tk.END, f"   Description: {description}\n")
        if tags:
            results_text_widget.insert(tk.END, f"   Tags: {tags}\n")
        results_text_widget.insert(tk.END, f"   Status: {production}\n\n")


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
