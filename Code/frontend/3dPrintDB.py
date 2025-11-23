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
CATEGORIES_URL = "http://localhost:8000/categories"

# Global variables
current_tags = []
tag_suggestions = []
categories = []
edit_mode = False
current_product_data = None
search_results = []
selected_category_id = None


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

    if not selected_category_id:
        messagebox.showerror("Error", "Please select a category")
        return

    # Build JSON payload
    payload = {
        "name": name,
        "description": description,
        "tags": current_tags.copy(),  # Use current tags list
        "production": production,
        "category_id": selected_category_id,
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
    """Search for products using unified search (empty query shows all products)"""
    global search_results

    # Get search query (allow empty for "show all")
    query = search_query.get().strip()

    # Build query parameters (empty q parameter will show all products)
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
        results_text.insert(tk.END, "No products found.")
        return

    # Check if this was a search with terms or showing all products
    has_search_terms = any(
        product.get("matches", {}).get("total", 0) > 0 for product in search_results
    )

    if has_search_terms:
        results_text.insert(
            tk.END, f"Found {len(search_results)} product(s) matching search:\n\n"
        )
    else:
        results_text.insert(
            tk.END, f"Showing all {len(search_results)} product(s) in database:\n\n"
        )

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
        # Show match details only for actual searches
        if has_search_terms and "matches" in product:
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

    # Check if there are any search results
    if not search_results:
        messagebox.showerror(
            "No Products Found",
            "There are no products to edit. Please perform a search first.",
        )
        return

    try:
        # Get the index from the entry field
        index_text = edit_index_entry.get().strip()
        if not index_text:
            messagebox.showwarning(
                "Index Required",
                "Please enter an index number (1, 2, 3, etc.) from the search results above.",
            )
            return

        try:
            index = int(index_text) - 1  # Convert to 0-based index
        except ValueError:
            messagebox.showerror(
                "Invalid Input", "Please enter a valid number for the index."
            )
            return

        if index < 0 or index >= len(search_results):
            messagebox.showerror(
                "Index Out of Range",
                f"The index {index_text} is not valid. Please enter a number between 1 and {len(search_results)}.",
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
        messagebox.showerror(
            "No Product Loaded",
            "No product is currently loaded for editing. Please use 'Load for Edit' first.",
        )
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

    if not edit_mode or not current_product_data:
        messagebox.showinfo("Info", "No active edit to discard.")
        return

    edit_mode = False
    current_product_data = None
    clear_edit_form()
    messagebox.showinfo(
        "Edit Discarded", "Changes have been discarded and the form has been cleared."
    )


# --- Category Management Functions ---
def load_categories():
    """Load categories from API"""
    global categories
    try:
        response = requests.get(CATEGORIES_URL)
        if response.status_code == 200:
            categories = response.json()
            update_category_dropdown()
        else:
            messagebox.showerror("Error", f"Failed to load categories: {response.text}")
    except Exception as e:
        messagebox.showerror("Error", f"Error loading categories: {str(e)}")


def update_category_dropdown():
    """Update the category dropdown with current categories"""
    category_combo["values"] = [
        f"{c['name']} ({c['sku_initials']})" for c in categories
    ]
    if categories:
        category_combo.current(0)  # Select first category by default


def create_new_category():
    """Create a new category via dialog"""
    # Create a dialog for new category
    dialog = tk.Toplevel(root)
    dialog.title("Create New Category")
    dialog.geometry("400x250")

    tk.Label(dialog, text="Category Name:").grid(
        row=0, column=0, sticky="e", padx=5, pady=5
    )
    name_entry = tk.Entry(dialog, width=30)
    name_entry.grid(row=0, column=1, padx=5, pady=5)

    tk.Label(dialog, text="SKU Initials (3 letters):").grid(
        row=1, column=0, sticky="e", padx=5, pady=5
    )
    initials_entry = tk.Entry(dialog, width=10)
    initials_entry.grid(row=1, column=1, sticky="w", padx=5, pady=5)

    tk.Label(dialog, text="Description:").grid(
        row=2, column=0, sticky="ne", padx=5, pady=5
    )
    desc_text = tk.Text(dialog, width=30, height=3)
    desc_text.grid(row=2, column=1, padx=5, pady=5)

    def save_category():
        name = name_entry.get().strip()
        initials = initials_entry.get().strip().upper()
        description = desc_text.get("1.0", tk.END).strip()

        if not name or not initials:
            messagebox.showerror("Error", "Name and SKU initials are required")
            return

        if len(initials) != 3 or not initials.isalpha():
            messagebox.showerror("Error", "SKU initials must be exactly 3 letters")
            return

        try:
            response = requests.post(
                CATEGORIES_URL,
                json={
                    "name": name,
                    "sku_initials": initials,
                    "description": description,
                },
            )

            if response.status_code == 200:
                messagebox.showinfo("Success", "Category created successfully")
                load_categories()  # Refresh categories
                dialog.destroy()
            else:
                messagebox.showerror(
                    "Error", f"Failed to create category: {response.text}"
                )
        except Exception as e:
            messagebox.showerror("Error", f"Error creating category: {str(e)}")

    def cancel():
        dialog.destroy()

    tk.Button(dialog, text="Create", command=save_category).grid(
        row=3, column=0, pady=10
    )
    tk.Button(dialog, text="Cancel", command=cancel).grid(row=3, column=1, pady=10)


def delete_category():
    """Delete selected category"""
    selected = category_combo.get()
    if not selected:
        messagebox.showwarning("Warning", "Please select a category to delete")
        return

    # Extract category name from selection
    category_name = selected.split(" (")[0]

    # Find category
    category = next((c for c in categories if c["name"] == category_name), None)
    if not category:
        messagebox.showerror("Error", "Category not found")
        return

    # Confirm deletion
    confirm = messagebox.askyesno(
        "Confirm Deletion",
        f"Are you sure you want to delete category:\n\n{category['name']} ({category['sku_initials']})\n\n"
        "This will only delete the category if no products are using it.",
    )

    if not confirm:
        return

    try:
        response = requests.delete(f"{CATEGORIES_URL}/{category['id']}")
        if response.status_code == 200:
            messagebox.showinfo("Success", "Category deleted successfully")
            load_categories()  # Refresh categories
        else:
            messagebox.showerror("Error", f"Failed to delete category: {response.text}")
    except Exception as e:
        messagebox.showerror("Error", f"Error deleting category: {str(e)}")


def on_category_select(event):
    """Handle category selection"""
    global selected_category_id
    selected = category_combo.get()
    if selected:
        category_name = selected.split(" (")[0]
        category = next((c for c in categories if c["name"] == category_name), None)
        if category:
            selected_category_id = category["id"]


def open_product_folder():
    """Open the product folder in file explorer"""
    # Check if there are any search results
    if not search_results:
        messagebox.showerror(
            "No Products Found",
            "There are no products to open folders for. Please perform a search first.",
        )
        return

    try:
        index_text = edit_index_entry.get().strip()
        if not index_text:
            messagebox.showwarning(
                "Index Required",
                "Please enter an index number (1, 2, 3, etc.) from the search results above.",
            )
            return

        try:
            index = int(index_text) - 1
        except ValueError:
            messagebox.showerror(
                "Invalid Input", "Please enter a valid number for the index."
            )
            return

        if index < 0 or index >= len(search_results):
            messagebox.showerror(
                "Index Out of Range",
                f"The index {index_text} is not valid. Please enter a number between 1 and {len(search_results)}.",
            )
            return

        product = search_results[index]
        sku = product["sku"]

        # Construct folder path (assuming standard structure)
        folder_path = f"/home/grbrum/Work/3d_print/Products/{sku}"

        # Check if folder exists
        import os

        if not os.path.exists(folder_path):
            messagebox.showerror(
                "Folder Not Found",
                f"The folder for product '{sku}' does not exist.\n\nExpected path: {folder_path}",
            )
            return

        # Open folder using system default file manager
        import subprocess
        import platform

        folder_opened = False

        # Try different file managers in order of preference
        file_managers = [
            ["nautilus", folder_path],  # GNOME Files
            ["thunar", folder_path],  # XFCE
            ["dolphin", folder_path],  # KDE
            ["pcmanfm", folder_path],  # LXDE
            ["caja", folder_path],  # MATE
            ["nemo", folder_path],  # Cinnamon
            ["xdg-open", folder_path],  # Fallback
        ]

        # Try system-specific commands for different platforms
        system = platform.system()
        if system == "Windows":
            file_managers.insert(0, ["explorer", folder_path])
        elif system == "Darwin":  # macOS
            file_managers.insert(0, ["open", folder_path])

        for cmd in file_managers:
            try:
                result = subprocess.run(
                    cmd, check=False, timeout=3, capture_output=True
                )
                if result.returncode == 0:
                    folder_opened = True
                    break
            except (
                subprocess.TimeoutExpired,
                FileNotFoundError,
                subprocess.SubprocessError,
            ):
                continue

        if folder_opened:
            messagebox.showinfo(
                "Folder Opened", f"Successfully opened folder for product {sku}"
            )
        else:
            # As a last resort, try Python's built-in file opening
            try:
                import webbrowser

                webbrowser.open(f"file://{folder_path}")
                messagebox.showinfo(
                    "Folder Opened",
                    f"Opened folder for product {sku} using web browser",
                )
            except:
                messagebox.showwarning(
                    "Could Not Open Folder",
                    f"Unable to automatically open the folder for product {sku}.\n\n"
                    f"Please manually navigate to:\n{folder_path}",
                )

    except Exception as e:
        messagebox.showerror("Error", f"Could not open folder: {str(e)}")


def delete_product():
    """Delete the selected product with confirmation"""
    # Check if there are any search results
    if not search_results:
        messagebox.showerror(
            "No Products Found",
            "There are no products to delete. Please perform a search first.",
        )
        return

    try:
        index_text = edit_index_entry.get().strip()
        if not index_text:
            messagebox.showwarning(
                "Index Required",
                "Please enter an index number (1, 2, 3, etc.) from the search results above.",
            )
            return

        try:
            index = int(index_text) - 1
        except ValueError:
            messagebox.showerror(
                "Invalid Input", "Please enter a valid number for the index."
            )
            return

        if index < 0 or index >= len(search_results):
            messagebox.showerror(
                "Index Out of Range",
                f"The index {index_text} is not valid. Please enter a number between 1 and {len(search_results)}.",
            )
            return

        product = search_results[index]
        sku = product["sku"]
        name = product["name"]

        # Confirmation dialog with options
        confirm = messagebox.askyesno(
            "Confirm Deletion",
            f"Are you sure you want to delete product:\n\nSKU: {sku}\nName: {name}\n\nThis action cannot be undone!",
        )

        if not confirm:
            return

        # Ask for deletion scope
        delete_choice = messagebox.askquestion(
            "Deletion Options",
            "Choose deletion method:\n\nYes = Delete from database AND file system\nNo = Delete from database only",
            icon="question",
        )

        delete_files = delete_choice == "yes"

        # Make API call to delete
        try:
            response = requests.delete(f"{API_URL}{sku}?delete_files={delete_files}")
            if response.status_code == 200:
                messagebox.showinfo(
                    "Success",
                    f"Product {sku} deleted successfully!\n\n"
                    f"Database: ✓ Deleted\n"
                    f"Files: {'✓ Deleted' if delete_files else '✗ Preserved'}",
                )
                # Refresh search results
                search_products()
            else:
                messagebox.showerror(
                    "Error", f"Failed to delete product\n{response.text}"
                )
        except Exception as e:
            messagebox.showerror("Error", f"Error deleting product: {str(e)}")

    except ValueError:
        messagebox.showerror("Error", "Please enter a valid number")
    except Exception as e:
        messagebox.showerror("Error", f"Error: {str(e)}")


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

# Check if we can actually display a GUI (catch tkinter errors)
try:
    # Force tkinter to initialize and check display
    root.update_idletasks()
except tk.TclError as e:
    print(
        f"ERROR: Cannot create GUI window. This appears to be a headless environment."
    )
    print(f"Tkinter error: {e}")
    print("To run this GUI application, you need a graphical desktop environment.")
    print("Try running from a terminal in your desktop environment, or use:")
    print("  export DISPLAY=:0  # or appropriate display number")
    root.destroy()
    import sys

    sys.exit(1)

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

# Category section
tk.Label(create_tab, text="Category:").grid(row=3, column=0, sticky="e", pady=5, padx=5)
category_frame = tk.Frame(create_tab)
category_frame.grid(row=3, column=1, columnspan=2, pady=5, padx=5, sticky="w")

category_combo = ttk.Combobox(category_frame, width=25, state="readonly")
category_combo.pack(side=tk.LEFT, padx=(0, 5))
category_combo.bind("<<ComboboxSelected>>", on_category_select)

tk.Button(category_frame, text="New Category", command=create_new_category).pack(
    side=tk.LEFT, padx=(0, 5)
)
tk.Button(
    category_frame, text="Delete Category", command=delete_category, fg="red"
).pack(side=tk.LEFT)

# Tags section
tk.Label(create_tab, text="Tags:").grid(row=4, column=0, sticky="ne", pady=5, padx=5)

# Tag input frame
tag_frame = tk.Frame(create_tab)
tag_frame.grid(row=4, column=1, columnspan=2, pady=5, padx=5, sticky="w")

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
tags_frame.grid(row=5, column=0, columnspan=3, pady=5, padx=5, sticky="w")

# Create tab buttons
create_button_frame = tk.Frame(create_tab)
create_button_frame.grid(row=6, column=0, columnspan=3, pady=10)

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
).pack(side=tk.LEFT, padx=(0, 5))
tk.Button(edit_controls_frame, text="Open Folder", command=open_product_folder).pack(
    side=tk.LEFT, padx=(0, 5)
)
tk.Button(
    edit_controls_frame, text="Delete Record", command=delete_product, fg="red"
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

# Load initial data
load_categories()

root.mainloop()
