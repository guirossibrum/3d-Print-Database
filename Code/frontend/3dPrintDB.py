#!/usr/bin/env python3

import tkinter as tk
from tkinter import ttk, messagebox, scrolledtext
import requests
import json
import threading
import time

# FastAPI endpoints
API_URL = "http://localhost:8000/products/"
TAGS_SUGGEST_URL = "http://localhost:8000/tags/suggest"
SEARCH_URL = "http://localhost:8000/products/search"

# Global variables
current_tags = []
tag_suggestions = []
edit_mode = False
current_product_data = None
search_results = []


def clear_form():
    entry_name.delete(0, tk.END)
    entry_description.delete(0, tk.END)
    var_production.set(True)
    current_tags.clear()
    update_tag_display()
    tag_combo.set("")


def cancel():
    root.destroy()


def add_tag():
    """Add a tag to the current tags list"""
    tag_text = tag_combo.get().strip()
    if tag_text and tag_text not in current_tags:
        current_tags.append(tag_text)
        update_tag_display()
        tag_combo.set("")  # Clear the input
        tag_combo.focus()  # Keep focus on input


def remove_tag(tag_to_remove):
    """Remove a tag from the current tags list"""
    if tag_to_remove in current_tags:
        current_tags.remove(tag_to_remove)
        update_tag_display()


def update_tag_display():
    """Update the display of current tags"""
    # Clear existing tag widgets
    for widget in tags_frame.winfo_children():
        widget.destroy()

    # Add current tags with remove buttons
    for i, tag in enumerate(current_tags):
        # Tag label
        tag_label = tk.Label(tags_frame, text=tag, bg="lightblue", padx=5, pady=2)
        tag_label.grid(row=i // 4, column=(i % 4) * 2, padx=2, pady=2, sticky="w")

        # Remove button
        remove_btn = tk.Button(
            tags_frame, text="×", font=("Arial", 8), command=lambda t=tag: remove_tag(t)
        )
        remove_btn.grid(row=i // 4, column=(i % 4) * 2 + 1, padx=2, pady=2)


def fetch_tag_suggestions(partial_tag, combo_widget=None):
    """Fetch tag suggestions from API in a separate thread"""

    def _fetch():
        try:
            if len(partial_tag.strip()) >= 2:
                response = requests.get(
                    f"{TAGS_SUGGEST_URL}?q={partial_tag.strip()}", timeout=2
                )
                if response.status_code == 200:
                    suggestions = [tag["name"] for tag in response.json()]
                    # Update suggestions on main thread
                    root.after(
                        0,
                        lambda: update_combo_values(
                            combo_widget or tag_combo, suggestions
                        ),
                    )
        except:
            pass  # Ignore network errors

    # Run in background thread
    threading.Thread(target=_fetch, daemon=True).start()


def update_combo_values(combo_widget, suggestions):
    """Update combobox values"""
    combo_widget["values"] = suggestions


def update_combo_suggestions(suggestions):
    """Update the combobox with suggestions"""
    global tag_suggestions
    tag_suggestions = suggestions
    tag_combo["values"] = tag_suggestions


def on_tag_typing(event):
    """Handle typing in the tag input field"""
    partial = tag_combo.get()
    if len(partial.strip()) >= 2:
        fetch_tag_suggestions(partial, tag_combo)


def create_item():
    name = entry_name.get().strip()
    description = entry_description.get().strip()
    production = var_production.get()

    if not name:
        messagebox.showerror("Error", "Name is required")
        return

    # Build JSON payload
    payload = {
        "name": name,
        "description": description,
        "tags": current_tags.copy(),  # Use current tags list
        "production": production,
    }

    try:
        response = requests.post(API_URL, json=payload)
        if response.status_code == 200:
            messagebox.showinfo(
                "Success", f"Product created: {response.json().get('sku')}"
            )
            clear_form()
        else:
            messagebox.showerror("Error", f"Failed to create product\n{response.text}")
    except Exception as e:
        messagebox.showerror("Error", str(e))


# --- Update/Search Functions ---
def search_products():
    """Search for products using unified search"""
    global search_results

    # Get search query
    query = search_query.get().strip()

    if not query:
        messagebox.showwarning("Warning", "Please enter a search query")
        return

    # Build query parameters
    params = {"q": query}

    try:
        response = requests.get(SEARCH_URL, params=params)
        if response.status_code == 200:
            search_results = response.json()
            display_search_results()
        else:
            messagebox.showerror("Error", f"Search failed\n{response.text}")
    except Exception as e:
        messagebox.showerror("Error", str(e))


def display_search_results():
    """Display search results in the text area"""
    results_text.delete(1.0, tk.END)

    if not search_results:
        results_text.insert(tk.END, "No products found matching the search criteria.")
        return

    results_text.insert(tk.END, f"Found {len(search_results)} product(s):\n\n")

    for i, product in enumerate(search_results, 1):
        results_text.insert(tk.END, f"{i}. SKU: {product['sku']}\n")
        results_text.insert(tk.END, f"   Name: {product['name']}\n")
        results_text.insert(
            tk.END, f"   Description: {product.get('description', 'N/A')}\n"
        )
        results_text.insert(tk.END, f"   Production: {product['production']}\n")
        results_text.insert(
            tk.END,
            f"   Tags: {', '.join(product['tags']) if product['tags'] else 'None'}\n",
        )
        # Show match details
        if "matches" in product:
            matches = product["matches"]
            results_text.insert(
                tk.END,
                f"   Matches: {matches['total']} total "
                f"({matches['name']} name, {matches['sku']} SKU, {matches['tags']} tag)\n",
            )
        results_text.insert(tk.END, "\n")


def load_product_for_edit():
    """Load selected product into edit form using index number"""
    global current_product_data, edit_mode

    try:
        # Get the index from the entry field
        index_text = edit_index_entry.get().strip()
        if not index_text:
            messagebox.showwarning(
                "Warning", "Please enter an index number (1, 2, 3, etc.)"
            )
            return

        try:
            index = int(index_text) - 1  # Convert to 0-based index
        except ValueError:
            messagebox.showerror("Error", "Please enter a valid number")
            return

        if index < 0 or index >= len(search_results):
            messagebox.showerror(
                "Error",
                f"Index out of range. Please enter a number between 1 and {len(search_results)}",
            )
            return

        # Get the product by index
        product = search_results[index]

        # Populate edit form
        current_product_data = product
        edit_name.delete(0, tk.END)
        edit_name.insert(0, product["name"])

        edit_description.delete(0, tk.END)
        edit_description.insert(0, product.get("description", ""))

        edit_var_production.set(product["production"])

        # Clear and populate tags
        edit_current_tags.clear()
        edit_current_tags.extend(product["tags"])
        update_edit_tag_display()

        edit_mode = True
        messagebox.showinfo("Success", f"Loaded product {product['sku']} for editing")

    except Exception as e:
        messagebox.showerror("Error", f"Failed to load product: {str(e)}")


def update_product():
    """Update the currently loaded product"""
    global current_product_data, edit_mode

    if not current_product_data or not edit_mode:
        messagebox.showerror("Error", "No product loaded for editing")
        return

    name = edit_name.get().strip()
    description = edit_description.get().strip()
    production = edit_var_production.get()

    if not name:
        messagebox.showerror("Error", "Name is required")
        return

    # Build update payload
    payload = {
        "name": name,
        "description": description,
        "tags": edit_current_tags.copy(),
        "production": production,
    }

    try:
        response = requests.put(f"{API_URL}{current_product_data['sku']}", json=payload)
        if response.status_code == 200:
            messagebox.showinfo(
                "Success", f"Product {current_product_data['sku']} updated successfully"
            )
            # Reset edit mode
            edit_mode = False
            current_product_data = None
            clear_edit_form()
        else:
            messagebox.showerror("Error", f"Failed to update product\n{response.text}")
    except Exception as e:
        messagebox.showerror("Error", str(e))


def clear_edit_form():
    """Clear the edit form"""
    edit_name.delete(0, tk.END)
    edit_description.delete(0, tk.END)
    edit_var_production.set(True)
    edit_current_tags.clear()
    update_edit_tag_display()


def discard_edit():
    """Discard current edit and reset form"""
    global edit_mode, current_product_data
    edit_mode = False
    current_product_data = None
    clear_edit_form()
    messagebox.showinfo("Info", "Edit discarded")


# --- Edit Tag Management Functions ---
edit_current_tags = []


def add_edit_tag():
    """Add a tag to the edit form"""
    tag_text = edit_tag_combo.get().strip()
    if tag_text and tag_text not in edit_current_tags:
        edit_current_tags.append(tag_text)
        update_edit_tag_display()
        edit_tag_combo.set("")


def remove_edit_tag(tag_to_remove):
    """Remove a tag from the edit form"""
    if tag_to_remove in edit_current_tags:
        edit_current_tags.remove(tag_to_remove)
        update_edit_tag_display()


def update_edit_tag_display():
    """Update the display of edit tags"""
    # Clear existing tag widgets
    for widget in edit_tags_frame.winfo_children():
        widget.destroy()

    # Add current tags with remove buttons
    for i, tag in enumerate(edit_current_tags):
        # Tag label
        tag_label = tk.Label(edit_tags_frame, text=tag, bg="lightgreen", padx=5, pady=2)
        tag_label.grid(row=i // 4, column=(i % 4) * 2, padx=2, pady=2, sticky="w")

        # Remove button
        remove_btn = tk.Button(
            edit_tags_frame,
            text="×",
            font=("Arial", 8),
            command=lambda t=tag: remove_edit_tag(t),
        )
        remove_btn.grid(row=i // 4, column=(i % 4) * 2 + 1, padx=2, pady=2)


# --- GUI ---
root = tk.Tk()
root.title("3D Print Database")
root.geometry("800x700")

# Create tabbed interface
tab_control = ttk.Notebook(root)

# Create tab frames
create_tab = ttk.Frame(tab_control)
update_tab = ttk.Frame(tab_control)

tab_control.add(create_tab, text="Create Product")
tab_control.add(update_tab, text="Update Product")
tab_control.pack(expand=1, fill="both")

# --- CREATE TAB ---
# Name field
tk.Label(create_tab, text="Name:").grid(row=0, column=0, sticky="e", padx=5, pady=5)
entry_name = tk.Entry(create_tab, width=30)
entry_name.grid(row=0, column=1, columnspan=2, pady=5, padx=5)

# Description field
tk.Label(create_tab, text="Description:").grid(
    row=1, column=0, sticky="e", padx=5, pady=5
)
entry_description = tk.Entry(create_tab, width=30)
entry_description.grid(row=1, column=1, columnspan=2, pady=5, padx=5)

# Production checkbox
var_production = tk.BooleanVar(value=True)
tk.Checkbutton(create_tab, text="Production", variable=var_production).grid(
    row=2, column=1, sticky="w", pady=5, padx=5
)

# Tags section
tk.Label(create_tab, text="Tags:").grid(row=3, column=0, sticky="ne", pady=5, padx=5)

# Tag input frame
tag_frame = tk.Frame(create_tab)
tag_frame.grid(row=3, column=1, columnspan=2, pady=5, padx=5, sticky="w")

# Tag input combobox with autocomplete
tag_combo = ttk.Combobox(tag_frame, width=25)
tag_combo.pack(side=tk.LEFT, padx=(0, 5))
tag_combo.bind("<KeyRelease>", on_tag_typing)
tag_combo.bind("<Return>", lambda e: add_tag())  # Enter key adds tag

# Add tag button
add_btn = tk.Button(tag_frame, text="Add Tag", command=add_tag)
add_btn.pack(side=tk.LEFT)

# Current tags display frame
tags_frame = tk.Frame(create_tab)
tags_frame.grid(row=4, column=0, columnspan=3, pady=5, padx=5, sticky="w")

# Create tab buttons
create_button_frame = tk.Frame(create_tab)
create_button_frame.grid(row=5, column=0, columnspan=3, pady=10)

tk.Button(create_button_frame, text="Clear", command=clear_form).pack(
    side=tk.LEFT, padx=5
)
tk.Button(create_button_frame, text="Cancel", command=cancel).pack(side=tk.LEFT, padx=5)
tk.Button(create_button_frame, text="Create Item", command=create_item).pack(
    side=tk.LEFT, padx=5
)

# --- UPDATE TAB ---
# Search section
search_frame = tk.LabelFrame(update_tab, text="Search Products", padx=10, pady=10)
search_frame.pack(fill="x", padx=10, pady=5)

tk.Label(search_frame, text="Search:").grid(row=0, column=0, sticky="e", padx=5, pady=2)
search_query = tk.Entry(search_frame, width=50)
search_query.grid(row=0, column=1, padx=5, pady=2)
tk.Label(search_frame, text="(searches name, SKU, and tags)").grid(
    row=0, column=2, padx=5, pady=2
)

tk.Button(search_frame, text="Search", command=search_products).grid(
    row=0, column=6, padx=10, pady=2
)

# Results section
results_frame = tk.LabelFrame(update_tab, text="Search Results", padx=10, pady=10)
results_frame.pack(fill="both", expand=True, padx=10, pady=5)

results_text = scrolledtext.ScrolledText(results_frame, height=8, wrap=tk.WORD)
results_text.pack(fill="both", expand=True)

# Load for edit controls
edit_controls_frame = tk.Frame(results_frame)
edit_controls_frame.pack(pady=5)

tk.Label(edit_controls_frame, text="Index:").pack(side=tk.LEFT, padx=(0, 5))
edit_index_entry = tk.Entry(edit_controls_frame, width=5)
edit_index_entry.pack(side=tk.LEFT, padx=(0, 10))
tk.Button(
    edit_controls_frame, text="Load for Edit", command=load_product_for_edit
).pack(side=tk.LEFT)

# Edit section
edit_frame = tk.LabelFrame(update_tab, text="Edit Product", padx=10, pady=10)
edit_frame.pack(fill="x", padx=10, pady=5)

# Edit form fields
tk.Label(edit_frame, text="Name:").grid(row=0, column=0, sticky="e", padx=5, pady=2)
edit_name = tk.Entry(edit_frame, width=30)
edit_name.grid(row=0, column=1, columnspan=2, pady=2, padx=5)

tk.Label(edit_frame, text="Description:").grid(
    row=1, column=0, sticky="e", padx=5, pady=2
)
edit_description = tk.Entry(edit_frame, width=30)
edit_description.grid(row=1, column=1, columnspan=2, pady=2, padx=5)

edit_var_production = tk.BooleanVar(value=True)
tk.Checkbutton(edit_frame, text="Production", variable=edit_var_production).grid(
    row=2, column=1, sticky="w", pady=2, padx=5
)

# Edit tags section
tk.Label(edit_frame, text="Tags:").grid(row=3, column=0, sticky="ne", pady=2, padx=5)

edit_tag_frame = tk.Frame(edit_frame)
edit_tag_frame.grid(row=3, column=1, columnspan=2, pady=2, padx=5, sticky="w")

edit_tag_combo = ttk.Combobox(edit_tag_frame, width=25)
edit_tag_combo.pack(side=tk.LEFT, padx=(0, 5))
edit_tag_combo.bind(
    "<KeyRelease>",
    lambda e: fetch_tag_suggestions(edit_tag_combo.get(), edit_tag_combo),
)
edit_tag_combo.bind("<Return>", lambda e: add_edit_tag())

edit_add_btn = tk.Button(edit_tag_frame, text="Add Tag", command=add_edit_tag)
edit_add_btn.pack(side=tk.LEFT)

edit_tags_frame = tk.Frame(edit_frame)
edit_tags_frame.grid(row=4, column=0, columnspan=3, pady=2, padx=5, sticky="w")

# Edit buttons
edit_button_frame = tk.Frame(update_tab)
edit_button_frame.pack(pady=10)

tk.Button(edit_button_frame, text="Update Product", command=update_product).pack(
    side=tk.LEFT, padx=5
)
tk.Button(edit_button_frame, text="Clear Edit", command=clear_edit_form).pack(
    side=tk.LEFT, padx=5
)
tk.Button(edit_button_frame, text="Discard Edit", command=discard_edit).pack(
    side=tk.LEFT, padx=5
)
tk.Button(edit_button_frame, text="Exit", command=cancel).pack(side=tk.LEFT, padx=5)

root.mainloop()
