#!/usr/bin/env python3

import tkinter as tk
from tkinter import ttk, messagebox, scrolledtext
import requests
import json
import threading
import time

# FastAPI endpoints
API_URL = "http://localhost:8000/products/"
TAGS_URL = "http://localhost:8000/tags"
TAGS_SUGGEST_URL = "http://localhost:8000/tags/suggest"
SEARCH_URL = "http://localhost:8000/products/search"
CATEGORIES_URL = "http://localhost:8000/categories"
INVENTORY_URL = "http://localhost:8000/inventory/status"

from modules.api_client import *
from modules import search


# Tag display functions (copied from modules for compatibility)
def update_tag_display(tags_list, display_frame, layout="pack"):
    """Update the display of tags with configurable layout"""
    # Clear existing
    for widget in display_frame.winfo_children():
        widget.destroy()

    if not tags_list:
        label = tk.Label(display_frame, text="(no tags)", fg="gray")
        if layout == "pack":
            label.pack(anchor="w")
        else:
            label.grid(row=0, column=0, sticky="w")
        return

    bg_color = "lightblue" if layout == "pack" else "lightgreen"

    for i, tag in enumerate(tags_list):
        tag_frame = tk.Frame(display_frame)

        if layout == "pack":
            tag_frame.pack(anchor="w", pady=1)
        else:
            tag_frame.grid(row=i // 4, column=(i % 4) * 2, padx=2, pady=2, sticky="w")

        tk.Label(tag_frame, text=tag, bg=bg_color, padx=5, pady=2).pack(side=tk.LEFT)

        remove_btn = tk.Button(
            tag_frame,
            text="×",
            font=("Arial", 8),
            command=lambda t=tag: remove_popup_tag(t, tags_list, display_frame)
            if layout == "grid"
            else remove_tag(t),
        )
        remove_btn.pack(side=tk.LEFT)


def add_popup_tag(widget, tags_list, display_frame, listbox=None):
    """Add a tag to the popup dialog"""
    tag_text = widget.get().strip()
    if tag_text and tag_text not in tags_list:
        tags_list.append(tag_text)
        update_tag_display(tags_list, display_frame, "grid")
        if hasattr(widget, "set"):
            widget.set("")
        else:
            widget.delete(0, tk.END)


def remove_popup_tag(tag_to_remove, tags_list, display_frame):
    """Remove a tag from the popup dialog"""
    if tag_to_remove in tags_list:
        tags_list.remove(tag_to_remove)
        update_tag_display(tags_list, display_frame, "grid")


def add_tag_from_listbox(listbox, current_tags, update_func):
    """Generic helper to add tag from listbox"""
    selection = listbox.curselection()
    if selection:
        tag = listbox.get(selection[0])
        if tag not in current_tags:
            current_tags.append(tag)
            update_func(current_tags)


def update_available_tags(new_tags_list):
    """Update available tags list and refresh listboxes"""
    global all_available_tags
    for tag in new_tags_list:
        if tag not in all_available_tags:
            all_available_tags.append(tag)
    all_available_tags.sort()
    # Update main listbox
    tag_listbox.delete(0, tk.END)
    for tag in all_available_tags:
        tag_listbox.insert(tk.END, tag)
    # Update edit listbox if exists
    if "edit_tag_listbox" in globals():
        edit_tag_listbox.delete(0, tk.END)
        for tag in all_available_tags:
            edit_tag_listbox.insert(tk.END, tag)


# Global variables
current_tags = []
tag_suggestions = []
all_available_tags = []  # All existing tags for the list
categories = []
edit_mode = False
current_product_data = None
search_results = []
selected_category_id = None


def on_time_focus_in(event):
    """Handle focus in for time entry field"""
    entry = event.widget
    current_text = entry.get()
    if current_text == "__:__":
        entry.delete(0, tk.END)
        entry.config(fg="black")


def on_time_focus_out(event):
    """Handle focus out for time entry field - complete formatting"""
    entry = event.widget
    current_text = entry.get().strip()

    # Check if it's empty or just underscores/colons
    if not current_text or current_text.replace("_", "").replace(":", "") == "":
        entry.delete(0, tk.END)
        entry.insert(0, "__:__")
        entry.config(fg="gray")
        return

    # If it has underscores, complete them intelligently
    if "_" in current_text:
        complete_time_with_underscores(entry, current_text)
        return

    # If it has colon, validate and format
    if ":" in current_text:
        parts = current_text.split(":")
        if len(parts) == 2:
            try:
                hours_str = parts[0].strip()
                minutes_str = parts[1].strip()

                hours = int(hours_str) if hours_str else 0
                minutes = int(minutes_str) if minutes_str else 0

                hours = min(hours, 23)
                minutes = min(minutes, 59)

                formatted = f"{hours:02d}:{minutes:02d}"
                if formatted != current_text:
                    entry.delete(0, tk.END)
                    entry.insert(0, formatted)
            except ValueError:
                # If parsing fails, try to complete based on digits
                complete_partial_time(entry, current_text)
        else:
            complete_partial_time(entry, current_text)
    else:
        # No colon, try to format as continuous digits
        complete_partial_time(entry, current_text)


def complete_time_with_underscores(entry, text):
    """Complete time that has underscores"""
    # Replace underscores with appropriate defaults
    if text == "__:__":
        return  # Already placeholder

    # Handle common patterns
    if text == "_:__":
        formatted = "00:00"
    elif text.endswith(":__"):
        # Has hours, missing minutes
        hours_part = text[:-3]  # Remove ":__"
        try:
            hours = int(hours_part)
            formatted = f"{hours:02d}:00"
        except ValueError:
            formatted = "00:00"
    elif text.endswith("_"):
        # Missing last digit
        base = text[:-1]  # Remove trailing "_"
        if ":" in base:
            # Has colon, missing minute digit
            parts = base.split(":")
            if len(parts) == 2:
                try:
                    hours = int(parts[0]) if parts[0] else 0
                    minutes = int(parts[1]) if parts[1] else 0
                    minutes = min(minutes, 59)
                    formatted = f"{hours:02d}:{minutes:02d}"
                except ValueError:
                    formatted = "00:00"
            else:
                formatted = "00:00"
        else:
            # No colon, assume it's HMM format
            digits = "".join(c for c in base if c.isdigit())
            if len(digits) == 3:
                hours = int(digits[:2])
                minutes = min(int(digits[2]), 59)
                formatted = f"{hours:02d}:{minutes:02d}"
            else:
                formatted = "00:00"
    else:
        # Fallback - extract digits and format
        digits = "".join(c for c in text if c.isdigit())
        if len(digits) >= 1:
            if len(digits) == 1:
                formatted = f"{digits}0:00"
            elif len(digits) == 2:
                formatted = f"{int(digits):02d}:00"
            elif len(digits) == 3:
                formatted = f"{int(digits[:2]):02d}:{int(digits[2]):02d}"
            else:
                formatted = f"{int(digits[:2]):02d}:{int(digits[2:4]):02d}"
        else:
            formatted = "__:__"
            entry.config(fg="gray")

    entry.delete(0, tk.END)
    entry.insert(0, formatted)
    if formatted != "__:__":
        entry.config(fg="black")


def complete_partial_time(entry, text):
    """Complete partial time entry"""
    digits = "".join(c for c in text if c.isdigit())

    if not digits:
        entry.delete(0, tk.END)
        entry.insert(0, "__:__")
        entry.config(fg="gray")
        return

    # Format based on digit count
    if len(digits) == 1:
        formatted = f"{digits}0:00"
    elif len(digits) == 2:
        formatted = f"{int(digits):02d}:00"
    elif len(digits) == 3:
        formatted = f"{int(digits[:2]):02d}:{int(digits[2]):02d}"
    else:
        formatted = f"{int(digits[:2]):02d}:{int(digits[2:4]):02d}"

    entry.delete(0, tk.END)
    entry.insert(0, formatted)
    entry.config(fg="black")


def format_time_complete(entry):
    """Complete time formatting by filling in missing parts"""
    current_text = entry.get()

    # Extract digits
    digits = "".join(c for c in current_text if c.isdigit())

    if not digits:
        entry.delete(0, tk.END)
        entry.insert(0, "__:__")
        entry.config(fg="gray")
        return

    # Format based on digit count
    if len(digits) == 1:
        formatted = f"{digits}0:00"
    elif len(digits) == 2:
        hour_int = int(digits)
        formatted = f"{hour_int:02d}:00"
    elif len(digits) == 3:
        hour_int = int(digits[:2])
        minute_int = int(digits[2])
        formatted = f"{hour_int:02d}:{minute_int:02d}"
    else:  # 4 or more digits
        hour_int = int(digits[:2])
        minute_int = min(int(digits[2:4]), 59)
        formatted = f"{hour_int:02d}:{minute_int:02d}"

    entry.delete(0, tk.END)
    entry.insert(0, formatted)
    entry.config(fg="black")


def format_time_input_live(entry):
    """Very conservative formatting - only help when clearly beneficial"""
    current_text = entry.get()

    # If it's the placeholder, don't format
    if current_text == "__:__":
        return

    # If the field is empty, don't format
    if not current_text.strip():
        return

    # Only format if we have exactly 4 digits and no colon (user typed continuous time)
    digit_count = sum(1 for c in current_text if c.isdigit())
    has_colon = ":" in current_text

    if digit_count == 4 and not has_colon:
        # User typed exactly 4 digits, format as HH:MM
        digits = "".join(c for c in current_text if c.isdigit())
        hour_int = int(digits[:2])
        minute_int = min(int(digits[2:]), 59)
        formatted = f"{hour_int:02d}:{minute_int:02d}"

        if formatted != current_text:
            entry.delete(0, tk.END)
            entry.insert(0, formatted)
            entry.icursor(len(formatted))  # Move cursor to end
    # Don't do any other formatting - let user edit freely


def on_time_key_release_popup(event):
    """Handle key release for time input field in popup"""
    entry = event.widget
    # Format after a short delay to allow for rapid typing
    entry.after(100, lambda: format_time_input_live(entry))


def format_time_input(entry, placeholder):
    """Format time input as hh:mm when user types digits"""
    current_text = entry.get().replace(":", "").replace(" ", "")

    # Remove placeholder if present
    if current_text == placeholder.replace(":", "").replace(" ", ""):
        current_text = ""

    # Only process if we have digits
    if current_text and current_text.isdigit():
        # Take only the first 4 digits
        digits = current_text[:4]

        if len(digits) >= 1:
            formatted = digits
            if len(digits) >= 2:
                formatted = digits[:2] + ":" + digits[2:]
            elif len(digits) == 1:
                formatted = digits

            # No hour limit - allow unlimited hours

            # Validate minutes (00-59)
            if ":" in formatted:
                parts = formatted.split(":")
                if len(parts) > 1:
                    try:
                        minutes = int(parts[1])
                        if minutes > 59:
                            formatted = parts[0] + ":59"
                    except:
                        pass

            entry.delete(0, tk.END)
            entry.insert(0, formatted)
            entry.config(fg="black")


def clear_form():
    entry_name.delete(0, tk.END)
    entry_description.delete(0, tk.END)
    var_production.set(True)
    current_tags.clear()
    update_tag_display(current_tags, tags_frame, "pack")
    tag_entry.delete(0, tk.END)


def cancel():
    root.destroy()


def add_tag():
    """Add a tag to the current tags list"""
    tag_text = tag_entry.get().strip()
    if tag_text and tag_text not in current_tags:
        current_tags.append(tag_text)
        update_tag_display(current_tags, tags_frame, "pack")
        tag_entry.delete(0, tk.END)  # Clear the input
        tag_entry.focus()
        # Add to available tags immediately
        global all_available_tags
        if tag_text not in all_available_tags:
            all_available_tags.append(tag_text)
            all_available_tags.sort()
            tag_listbox.delete(0, tk.END)
            for tag in all_available_tags:
                tag_listbox.insert(tk.END, tag)


def remove_tag(tag_to_remove):
    """Remove a tag from the current tags list"""
    if tag_to_remove in current_tags:
        current_tags.remove(tag_to_remove)
        update_tag_display(current_tags, tags_frame, "pack")


# Removed autocomplete functions - using list-based tag selection now


def load_all_tags_for_list():
    """Load all existing tags to populate the listbox"""
    global all_available_tags

    try:
        response = requests.get(TAGS_URL, timeout=5)  # Synchronous for debugging
        if response.status_code == 200:
            data = response.json()
            all_available_tags = sorted([tag["name"] for tag in data if "name" in tag])
            # Update listbox immediately
            filter_tag_list()
        else:
            # Show error for debugging
            messagebox.showerror(
                "Tags Error",
                f"Failed to load tags: {response.status_code} - {response.text[:200]}",
            )
    except Exception as e:
        # Show error for debugging
        messagebox.showerror("Tags Error", f"Error loading tags: {str(e)}")


def filter_tag_list(event=None):
    """Filter the tag list based on input text"""
    filter_text = tag_entry.get().strip().lower()

    # Clear current list
    tag_listbox.delete(0, tk.END)

    # Filter and add matching tags
    for tag in all_available_tags:
        if not filter_text or filter_text in tag.lower():
            tag_listbox.insert(tk.END, tag)


def add_tag_from_list(event=None):
    """Add selected tag from the listbox"""
    add_tag_from_listbox(
        tag_listbox,
        current_tags,
        lambda tags: update_tag_display(tags, tags_frame, "pack"),
    )
    tag_entry.delete(0, tk.END)  # Clear input
    tag_entry.focus()


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
            messagebox.showerror(
                "Cannot Delete",
                f"Tag '{selected_tag}' is still used by products and cannot be deleted.",
            )
        else:
            messagebox.showerror("Error", f"Failed to delete tag: {response.text}")
    except Exception as e:
        messagebox.showerror("Error", f"Error deleting tag: {str(e)}")
        # No need to refresh list since we're using existing tags


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
            update_available_tags(current_tags)
            clear_form()
        else:
            messagebox.showerror("Error", f"Failed to create product\n{response.text}")
    except Exception as e:
        messagebox.showerror("Error", str(e))


# --- Update/Search Functions ---
def search_products():
    """Search for products using unified search (empty query shows all products)"""
    search.search_products(search_query, results_text, search_results)


def show_edit_callback(product):
    """Callback to show edit dialog and set dialog_open"""
    global dialog_open
    dialog_open = True
    show_edit_product_dialog(product)


# load_product_for_edit function removed - now using popup dialogs


# update_product function removed - now using popup dialogs


# clear_edit_form function removed - now using popup dialogs


# discard_edit function removed - now using popup dialogs


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
    global selected_category_id
    category_combo["values"] = [
        f"{c['name']} ({c['sku_initials']})" for c in categories
    ]
    if categories:
        category_combo.current(0)  # Select first category by default
        selected_category_id = categories[0][
            "id"
        ]  # Set the ID for the selected category


def create_new_category():
    """Create a new category via dialog"""
    # Create a dialog for new category
    dialog = tk.Toplevel(root)
    dialog.title("Create New Category")
    dialog.geometry("400x250")

    def on_dialog_close():
        dialog.destroy()

    dialog.protocol("WM_DELETE_WINDOW", on_dialog_close)

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
            create_category_via_api(name, initials, description)
            messagebox.showinfo("Success", "Category created successfully")
            load_categories()  # Refresh categories

            # Auto-select the newly created category
            new_category_name = name
            for i, cat in enumerate(categories):
                if cat["name"] == new_category_name:
                    category_combo.current(i)
                    on_category_select(None)  # Trigger selection handler
                    break

            dialog.destroy()
        except Exception as e:
            messagebox.showerror("Error", f"Error creating category: {str(e)}")

    def cancel():
        dialog.destroy()

    tk.Button(dialog, text="Create", command=save_category).grid(
        row=3, column=0, pady=10
    )
    tk.Button(dialog, text="Cancel", command=cancel).grid(row=3, column=1, pady=10)

    # Make dialog modal
    dialog.transient(root)
    dialog.grab_set()
    root.wait_window(dialog)


def edit_category():
    """Edit selected category via dialog"""
    selected = category_combo.get()
    if not selected:
        messagebox.showwarning("Warning", "Please select a category to edit")
        return

    # Extract category name from selection
    category_name = selected.split(" (")[0]

    # Find category
    category = next((c for c in categories if c["name"] == category_name), None)
    if not category:
        messagebox.showerror("Error", "Category not found")
        return

    # Create a dialog for editing category
    dialog = tk.Toplevel(root)
    dialog.title("Edit Category")
    dialog.geometry("400x250")

    def on_dialog_close():
        dialog.destroy()

    dialog.protocol("WM_DELETE_WINDOW", on_dialog_close)

    tk.Label(dialog, text="Category Name:").grid(
        row=0, column=0, sticky="e", padx=5, pady=5
    )
    name_entry = tk.Entry(dialog, width=30)
    name_entry.insert(0, category["name"])
    name_entry.grid(row=0, column=1, padx=5, pady=5)

    tk.Label(dialog, text="SKU Initials (3 letters):").grid(
        row=1, column=0, sticky="e", padx=5, pady=5
    )
    initials_entry = tk.Entry(dialog, width=10)
    initials_entry.insert(0, category["sku_initials"])
    initials_entry.grid(row=1, column=1, sticky="w", padx=5, pady=5)

    tk.Label(dialog, text="Description:").grid(
        row=2, column=0, sticky="ne", padx=5, pady=5
    )
    desc_text = tk.Text(dialog, width=30, height=3)
    desc_text.insert("1.0", category.get("description", ""))
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
            update_category_via_api(category["id"], name, initials, description)
            messagebox.showinfo("Success", "Category updated successfully")
            load_categories()  # Refresh categories

            # Update the selection to the edited category
            updated_name = name
            for i, cat in enumerate(categories):
                if cat["id"] == category["id"]:
                    category_combo.current(i)
                    on_category_select(None)  # Trigger selection handler
                    break

            dialog.destroy()
        except Exception as e:
            messagebox.showerror("Error", f"Error updating category: {str(e)}")

    def cancel():
        dialog.destroy()

    tk.Button(dialog, text="Save", command=save_category).grid(row=3, column=0, pady=10)
    tk.Button(dialog, text="Cancel", command=cancel).grid(row=3, column=1, pady=10)

    # Make dialog modal
    dialog.transient(root)
    dialog.grab_set()
    root.wait_window(dialog)


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
            # Clear current selection before refreshing
            category_combo.set("")
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


# open_product_folder function updated to work with popup dialogs


# delete_product function removed - now in popup dialogs


# Edit tag management functions removed - now using popup dialogs


# --- Inventory Management Functions ---
def load_inventory_status():
    """Load and display inventory status for all products"""
    try:
        response = requests.get(INVENTORY_URL)
        if response.status_code == 200:
            inventory_data = response.json()

            # Clear existing items
            for item in inventory_tree.get_children():
                inventory_tree.delete(item)

            # Add inventory items
            total_value = 0
            low_stock_count = 0
            out_of_stock_count = 0

            for item in inventory_data:
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
            messagebox.showerror("Error", f"Failed to load inventory: {response.text}")
    except Exception as e:
        messagebox.showerror("Error", f"Error loading inventory: {str(e)}")


# Global flag to prevent multiple dialogs
dialog_open = False


def load_product_from_search():
    """Load product from search results for editing (double-click)"""
    search.load_product_from_search(results_text, search_results, show_edit_callback)


def add_popup_tag(widget, tags_list, display_frame, listbox=None):
    """Add a tag to the popup dialog"""
    tag_text = widget.get().strip()
    if tag_text and tag_text not in tags_list:
        tags_list.append(tag_text)
        update_tag_display(tags_list, display_frame, "grid")
        if hasattr(widget, "set"):
            widget.set("")
        else:
            widget.delete(0, tk.END)

        # Add to available tags if new
        global all_available_tags
        if tag_text not in all_available_tags:
            all_available_tags.append(tag_text)
            all_available_tags.sort()
            if listbox:
                listbox.delete(0, tk.END)
                for tag in all_available_tags:
                    listbox.insert(tk.END, tag)


def remove_popup_tag(tag_to_remove, tags_list, display_frame):
    """Remove a tag from the popup dialog"""
    if tag_to_remove in tags_list:
        tags_list.remove(tag_to_remove)
        update_tag_display(tags_list, display_frame, "grid")


def add_tag_from_listbox(listbox, current_tags, update_func):
    """Generic helper to add tag from listbox"""
    selection = listbox.curselection()
    if selection:
        tag = listbox.get(selection[0])
        if tag not in current_tags:
            current_tags.append(tag)
            update_func(current_tags)


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


def show_edit_product_dialog(product):
    """Show popup dialog for editing a product"""
    global edit_current_tags, current_product_data, edit_mode

    # Refresh available tags from database
    load_all_tags_for_list()

    # Set global state
    current_product_data = product
    edit_mode = True
    edit_current_tags = product.get("tags", []).copy()

    # Create edit dialog
    dialog = tk.Toplevel(root)
    dialog.title(f"Edit Product - {product['sku']}")
    dialog.geometry("600x700")

    # Product info header
    header_frame = tk.Frame(dialog)
    header_frame.pack(pady=10, padx=10, fill="x")

    tk.Label(
        header_frame, text=f"SKU: {product['sku']}", font=("Arial", 12, "bold")
    ).pack(anchor="w")
    tk.Label(header_frame, text=f"Name: {product['name']}", font=("Arial", 10)).pack(
        anchor="w"
    )

    # Create main frame for form
    main_frame = tk.Frame(dialog)

    # Form fields
    # Name
    tk.Label(main_frame, text="Name:").grid(row=0, column=0, sticky="e", padx=5, pady=5)
    edit_name = tk.Entry(main_frame, width=50)
    name_value = product.get("name", "")
    if name_value is not None:
        edit_name.insert(0, str(name_value))
    edit_name.grid(row=0, column=1, columnspan=3, pady=5, padx=5, sticky="w")

    # Description
    tk.Label(main_frame, text="Description:").grid(
        row=1, column=0, sticky="e", padx=5, pady=5
    )
    edit_description = tk.Entry(main_frame, width=50)
    desc_value = product.get("description")
    if desc_value is not None:
        edit_description.insert(0, str(desc_value))
    edit_description.grid(row=1, column=1, columnspan=3, pady=5, padx=5, sticky="w")

    # Production checkbox
    edit_var_production = tk.BooleanVar(value=product["production"])
    tk.Checkbutton(
        main_frame, text="Production Ready", variable=edit_var_production
    ).grid(row=2, column=1, sticky="w", pady=5, padx=5)

    # Material and Color
    tk.Label(main_frame, text="Material:").grid(
        row=3, column=0, sticky="e", padx=5, pady=2
    )
    edit_material = tk.Entry(main_frame, width=20)
    material_value = product.get("material")
    if material_value is not None:
        edit_material.insert(0, str(material_value))
    edit_material.grid(row=3, column=1, pady=2, padx=5, sticky="w")

    tk.Label(main_frame, text="Color:").grid(
        row=3, column=2, sticky="e", padx=5, pady=2
    )
    edit_color = tk.Entry(main_frame, width=20)
    color_value = product.get("color")
    if color_value is not None:
        edit_color.insert(0, str(color_value))
    edit_color.grid(row=3, column=3, pady=2, padx=5, sticky="w")

    # Print time and Weight
    tk.Label(main_frame, text="Print Time:").grid(
        row=4, column=0, sticky="e", padx=5, pady=2
    )
    edit_print_time = tk.Entry(main_frame, width=20)
    print_time_value = product.get("print_time")
    if print_time_value is not None and str(print_time_value).strip():
        # Format existing time value
        try:
            # Try to parse and format the existing time
            time_str = str(print_time_value).strip()
            if ":" in time_str:
                parts = time_str.split(":")
                if len(parts) == 2:
                    hours = parts[0].zfill(2)
                    minutes = parts[1].zfill(2)
                    edit_print_time.insert(0, f"{hours}:{minutes}")
                else:
                    edit_print_time.insert(0, "__:__")
            else:
                edit_print_time.insert(0, "__:__")
        except:
            edit_print_time.insert(0, "__:__")
        edit_print_time.config(fg="black")
    else:
        edit_print_time.insert(0, "__:__")
        edit_print_time.config(fg="gray")
    edit_print_time.bind("<FocusIn>", lambda e: on_time_focus_in(e))
    edit_print_time.bind("<FocusOut>", lambda e: on_time_focus_out(e))
    edit_print_time.bind("<KeyRelease>", on_time_key_release_popup)
    edit_print_time.grid(row=4, column=1, pady=2, padx=5, sticky="w")

    tk.Label(main_frame, text="Weight (g):").grid(
        row=4, column=2, sticky="e", padx=5, pady=2
    )
    edit_weight = tk.Entry(main_frame, width=20)
    weight_value = product.get("weight")
    if weight_value is not None:
        edit_weight.insert(0, str(weight_value))
    edit_weight.grid(row=4, column=3, pady=2, padx=5, sticky="w")

    # Tags section
    tk.Label(main_frame, text="Tags:").grid(
        row=5, column=0, sticky="ne", pady=5, padx=5
    )

    edit_tag_frame = tk.Frame(main_frame)
    edit_tag_frame.grid(row=5, column=1, columnspan=3, pady=5, padx=5, sticky="w")

    edit_tag_entry = tk.Entry(edit_tag_frame, width=30)
    edit_tag_entry.pack(side=tk.LEFT, padx=(0, 5))

    edit_add_btn = tk.Button(
        edit_tag_frame,
        text="Add Tag",
        command=lambda: add_popup_tag(
            edit_tag_entry, edit_current_tags, edit_tags_frame, edit_tag_listbox
        ),
    )
    edit_add_btn.pack(side=tk.LEFT)

    edit_tags_frame = tk.Frame(main_frame)
    edit_tags_frame.grid(row=6, column=0, columnspan=4, pady=5, padx=5, sticky="w")

    # Initialize tag display
    update_tag_display(edit_current_tags, edit_tags_frame, "grid")

    # Available tags list
    tk.Label(main_frame, text="Available Tags:").grid(
        row=7, column=0, sticky="ne", pady=5, padx=5
    )

    edit_tag_list_frame = tk.Frame(main_frame)
    edit_tag_list_frame.grid(row=7, column=1, columnspan=3, pady=5, padx=5, sticky="w")

    edit_tag_listbox = tk.Listbox(edit_tag_list_frame, height=6, width=40)
    edit_tag_listbox.pack(side=tk.LEFT, fill=tk.BOTH, expand=True)

    edit_tag_scrollbar = tk.Scrollbar(edit_tag_list_frame)
    edit_tag_scrollbar.pack(side=tk.RIGHT, fill=tk.Y)

    edit_tag_listbox.config(yscrollcommand=edit_tag_scrollbar.set)
    edit_tag_scrollbar.config(command=edit_tag_listbox.yview)

    # Populate listbox
    for tag in all_available_tags:
        edit_tag_listbox.insert(tk.END, tag)

    # Bind double-click to add
    edit_tag_listbox.bind(
        "<Double-1>",
        lambda e: add_tag_from_listbox(
            edit_tag_listbox,
            edit_current_tags,
            lambda tags: update_tag_display(tags, edit_tags_frame, "grid"),
        ),
    )

    # Action buttons
    button_frame = tk.Frame(dialog)
    button_frame.pack(pady=20)

    def save_changes():
        """Save the edited product"""
        try:
            name = edit_name.get().strip()
            description = edit_description.get().strip()
            production = edit_var_production.get()
            material = edit_material.get().strip()
            color = edit_color.get().strip()
            print_time = edit_print_time.get().strip()
            weight_text = edit_weight.get().strip()

            if not name:
                messagebox.showerror("Error", "Name is required")
                return

            # Build payload - only allow existing tags for edit
            allowed_tags = set(all_available_tags + product.get("tags", []))
            filtered_tags = [t for t in edit_current_tags if t in allowed_tags]
            payload = {
                "name": name,
                "description": description,
                "tags": filtered_tags,
                "production": production,
                "material": material or None,
                "color": color or None,
                "print_time": print_time or None,
                "weight": int(weight_text) if weight_text else None,
            }

            # Update product
            save_product_changes(product["sku"], payload)
            # Add new tags to available tags (not saved to DB)
            original_tags = product.get("tags", [])
            new_tags = [t for t in edit_current_tags if t not in original_tags]
            update_available_tags(new_tags)
            global dialog_open
            dialog_open = False
            dialog.destroy()
            # Refresh search results
            search_products()

        except Exception as e:
            messagebox.showerror("Error", f"Error updating product: {str(e)}")

    def open_folder():
        """Open the product folder"""
        try:
            # Find the product in search results to get folder path
            folder_path = f"/home/grbrum/Work/3d_print/Products/{product['sku']}"

            import os

            if not os.path.exists(folder_path):
                messagebox.showerror(
                    "Folder Not Found",
                    f"The folder for product '{product['sku']}' does not exist.",
                )
                return

            # Open folder using system default
            import subprocess
            import platform

            system = platform.system()
            if system == "Linux":
                # Try different file managers for Linux
                file_managers = ["dolphin", "nautilus", "thunar", "pcmanfm", "nemo"]
                opened = False
                for fm in file_managers:
                    try:
                        subprocess.Popen(
                            [fm, folder_path],
                            stdout=subprocess.DEVNULL,
                            stderr=subprocess.DEVNULL,
                        )
                        opened = True
                        break
                    except FileNotFoundError:
                        continue
                if not opened:
                    # Fallback to xdg-open
                    subprocess.Popen(
                        ["xdg-open", folder_path],
                        stdout=subprocess.DEVNULL,
                        stderr=subprocess.DEVNULL,
                    )
            elif system == "Darwin":  # macOS
                subprocess.Popen(
                    ["open", folder_path],
                    stdout=subprocess.DEVNULL,
                    stderr=subprocess.DEVNULL,
                )
            elif system == "Windows":
                subprocess.Popen(
                    ["explorer", folder_path],
                    stdout=subprocess.DEVNULL,
                    stderr=subprocess.DEVNULL,
                )

        except Exception as e:
            messagebox.showerror("Error", f"Could not open folder: {str(e)}")

    def delete_record():
        """Delete the product record"""
        if not messagebox.askyesno(
            "Confirm Deletion",
            f"Are you sure you want to delete product:\n\nSKU: {product['sku']}\nName: {product['name']}\n\nThis action cannot be undone!",
        ):
            return

        try:
            # Ask for deletion scope
            delete_choice = messagebox.askquestion(
                "Deletion Options",
                "Choose deletion method:\n\nYes = Delete from database AND file system\nNo = Delete from database only",
                icon="question",
            )

            delete_files = delete_choice == "yes"

            # Delete product
            response = requests.delete(
                f"{API_URL}{product['sku']}?delete_files={delete_files}"
            )
            if response.status_code == 200:
                messagebox.showinfo(
                    "Success", f"Product {product['sku']} deleted successfully!"
                )
                global dialog_open
                dialog_open = False
                dialog.destroy()
                # Refresh search results
                search_products()
            else:
                messagebox.showerror(
                    "Error", f"Failed to delete product: {response.text}"
                )

        except Exception as e:
            messagebox.showerror("Error", f"Error deleting product: {str(e)}")

    tk.Button(
        button_frame, text="Save Changes", command=save_changes, bg="lightgreen"
    ).pack(side=tk.LEFT, padx=5)
    tk.Button(button_frame, text="Open Folder", command=open_folder).pack(
        side=tk.LEFT, padx=5
    )
    tk.Button(button_frame, text="Delete Record", command=delete_record, fg="red").pack(
        side=tk.LEFT, padx=5
    )

    def cancel_edit_dialog():
        global dialog_open
        dialog_open = False
        dialog.destroy()

    tk.Button(button_frame, text="Cancel", command=cancel_edit_dialog).pack(
        side=tk.LEFT, padx=5
    )

    # Pack main frame
    main_frame.pack(fill="both", expand=True, padx=10, pady=10)

    # Reset flag when dialog closes
    def on_dialog_close():
        global dialog_open
        dialog_open = False
        dialog.destroy()

    dialog.protocol("WM_DELETE_WINDOW", on_dialog_close)

    # Make dialog visible first, then modal
    dialog.update_idletasks()
    dialog.transient(root)
    dialog.grab_set()
    root.wait_window(dialog)


def adjust_inventory_dialog():
    """Simple dialog for quick inventory adjustments"""
    selected_item = inventory_tree.selection()
    if not selected_item:
        messagebox.showwarning(
            "No Selection", "Please double-click on a product to adjust inventory."
        )
        return

    # Get selected product data
    item_values = inventory_tree.item(selected_item[0], "values")
    sku = item_values[0]
    product_name = item_values[1]
    current_stock = int(item_values[2]) if item_values[2].isdigit() else 0

    # Create simple adjustment dialog
    dialog = tk.Toplevel(root)
    dialog.title(f"Adjust Inventory - {sku}")
    dialog.geometry("350x250")

    # Product info
    tk.Label(dialog, text=f"Product: {product_name}", font=("Arial", 10, "bold")).pack(
        pady=10
    )
    tk.Label(dialog, text=f"Current Stock: {current_stock}").pack(pady=5)

    # Quantity input
    tk.Label(dialog, text="Quantity:").pack(pady=5)
    quantity_entry = tk.Entry(dialog, width=10, justify="center")
    quantity_entry.insert(0, "1")
    quantity_entry.pack(pady=5)
    quantity_entry.focus()

    # Operation selection
    operation_var = tk.StringVar(value="printed")
    operation_frame = tk.Frame(dialog)
    operation_frame.pack(pady=10)

    tk.Radiobutton(
        operation_frame, text="Printed (Add)", variable=operation_var, value="printed"
    ).pack(side=tk.LEFT, padx=10)
    tk.Radiobutton(
        operation_frame, text="Sold (Remove)", variable=operation_var, value="sold"
    ).pack(side=tk.LEFT, padx=10)

    def adjust_stock():
        """Adjust stock based on selected operation"""
        try:
            quantity = int(quantity_entry.get().strip())
            if quantity <= 0:
                raise ValueError("Quantity must be positive")

            operation = operation_var.get()

            # Apply adjustment
            success_message = apply_inventory_adjustment(
                sku, operation, quantity, current_stock
            )

            messagebox.showinfo("Success", success_message)
            global dialog_open
            dialog_open = False
            dialog.destroy()
            load_inventory_status()  # Refresh inventory display

        except ValueError as e:
            messagebox.showerror("Invalid Input", str(e))
        except Exception as e:
            messagebox.showerror("Error", f"Error updating inventory: {str(e)}")

    # Buttons
    button_frame = tk.Frame(dialog)
    button_frame.pack(pady=20)

    def cancel_inventory_dialog():
        global dialog_open
        dialog_open = False
        dialog.destroy()

    tk.Button(button_frame, text="Apply", command=adjust_stock, bg="lightgreen").pack(
        side=tk.LEFT, padx=5
    )
    tk.Button(button_frame, text="Cancel", command=cancel_inventory_dialog).pack(
        side=tk.LEFT, padx=5
    )

    # Reset flag when dialog closes
    def on_inventory_dialog_close():
        global dialog_open
        dialog_open = False
        dialog.destroy()

    dialog.protocol("WM_DELETE_WINDOW", on_inventory_dialog_close)

    # Make dialog modal
    dialog.transient(root)
    dialog.grab_set()
    root.wait_window(dialog)


# --- GUI ---
root = tk.Tk()
root.title("3D Print Database")
root.geometry("1000x800")  # Made wider for inventory tab

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
inventory_tab = ttk.Frame(tab_control)

tab_control.add(create_tab, text="Create Product")
tab_control.add(update_tab, text="Update Product")
tab_control.add(inventory_tab, text="Inventory")


# Refresh available tags when switching tabs
def on_tab_change(event):
    selected = tab_control.index(tab_control.select())
    if selected == 0:  # Create Product tab
        load_all_tags_for_list()


tab_control.bind("<<NotebookTabChanged>>", on_tab_change)
tab_control.pack(expand=1, fill="both")


# Tab change handler
def on_tab_change(event):
    """Handle tab selection changes"""
    selected_tab = tab_control.select()
    tab_text = tab_control.tab(selected_tab, "text")

    if tab_text == "Create Product":
        # Load all existing tags for the list when Create Product tab is selected
        load_all_tags_for_list()
    elif tab_text == "Update Product":
        # Auto-load all products when Update Product tab is selected
        search_query.delete(0, tk.END)  # Clear search field
        search_products()  # Load all products
    elif tab_text == "Inventory":
        # Auto-load inventory status when Inventory tab is selected
        load_inventory_status()


# Bind tab change event
tab_control.bind("<<NotebookTabChanged>>", on_tab_change)

# --- CREATE TAB ---
# Name field (short field, most important)
tk.Label(create_tab, text="Name:").grid(row=0, column=0, sticky="e", padx=5, pady=5)
entry_name = tk.Entry(create_tab, width=50)
entry_name.grid(row=0, column=1, columnspan=3, pady=5, padx=5, sticky="w")

# Description field (longer field)
tk.Label(create_tab, text="Description:").grid(
    row=1, column=0, sticky="e", padx=5, pady=5
)
entry_description = tk.Entry(create_tab, width=50)
entry_description.grid(row=1, column=1, columnspan=3, pady=5, padx=5, sticky="w")

# Category section (important for SKU generation)
tk.Label(create_tab, text="Category:").grid(row=2, column=0, sticky="e", padx=5, pady=5)
category_frame = tk.Frame(create_tab)
category_frame.grid(row=2, column=1, columnspan=3, pady=5, padx=5, sticky="w")

category_combo = ttk.Combobox(category_frame, width=25, state="readonly")
category_combo.pack(side=tk.LEFT, padx=(0, 5))
category_combo.bind("<<ComboboxSelected>>", on_category_select)

tk.Button(category_frame, text="New", command=create_new_category).pack(
    side=tk.LEFT, padx=(0, 5)
)
tk.Button(category_frame, text="Edit", command=edit_category).pack(
    side=tk.LEFT, padx=(0, 5)
)
tk.Button(category_frame, text="Delete", command=delete_category, fg="red").pack(
    side=tk.LEFT
)

# Production checkbox
var_production = tk.BooleanVar(value=True)
tk.Checkbutton(create_tab, text="Production Ready", variable=var_production).grid(
    row=3, column=1, sticky="w", pady=5, padx=5
)


# Tags section
tk.Label(create_tab, text="Tags:").grid(row=8, column=0, sticky="ne", pady=5, padx=5)

# Tag input frame (left side)
tag_input_frame = tk.Frame(create_tab)
tag_input_frame.grid(row=8, column=1, pady=5, padx=5, sticky="w")

# Tag input entry (simple text field)
tag_entry = tk.Entry(tag_input_frame, width=25)
tag_entry.pack(side=tk.LEFT, padx=(0, 5))
tag_entry.bind("<KeyRelease>", filter_tag_list)

# Add tag button
add_btn = tk.Button(tag_input_frame, text="Add Tag", command=add_tag)
add_btn.pack(side=tk.LEFT)

# Available tags list (positioned to the right, spanning more columns)
tag_list_frame = tk.Frame(create_tab)
tag_list_frame.grid(row=8, column=4, columnspan=2, pady=5, padx=5, sticky="w")

tk.Label(tag_list_frame, text="Available Tags:").pack(anchor="w")
tag_listbox = tk.Listbox(tag_list_frame, width=25, height=10, selectmode=tk.SINGLE)
tag_listbox.pack()
tag_listbox.bind("<Double-1>", add_tag_from_list)

# Delete tag button
delete_tag_btn = tk.Button(tag_list_frame, text="Delete Tag", command=delete_unused_tag)
delete_tag_btn.pack(pady=(5, 0))

# Current tags display frame
tags_frame = tk.Frame(create_tab)
tags_frame.grid(row=9, column=0, columnspan=4, pady=5, padx=5, sticky="w")

# Create tab buttons
create_button_frame = tk.Frame(create_tab)
create_button_frame.grid(row=10, column=0, columnspan=4, pady=10)

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
search_query.bind("<KeyRelease>", lambda e: search_products())  # Active filtering
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

# Bind double-click to load product for editing
results_text.bind("<Double-1>", lambda e: load_product_from_search())

# Edit controls moved to popup dialogs - double-click products to edit

# Edit instructions
edit_info_frame = tk.LabelFrame(update_tab, text="Edit Product", padx=10, pady=10)
edit_info_frame.pack(fill="x", padx=10, pady=5)

tk.Label(
    edit_info_frame,
    text="Double-click on any product in the search results above to edit it.",
    font=("Arial", 10),
).pack(pady=5)
tk.Label(
    edit_info_frame,
    text="The edit form will open in a popup window with all product details.",
    font=("Arial", 9),
    fg="gray",
).pack(pady=2)

# --- INVENTORY TAB ---
# Inventory list section
inventory_frame = tk.LabelFrame(
    inventory_tab, text="Inventory Status", padx=10, pady=10
)
inventory_frame.pack(fill="both", expand=True, padx=10, pady=5)

# Inventory controls
inventory_controls_frame = tk.Frame(inventory_frame)
inventory_controls_frame.pack(pady=5)

tk.Label(
    inventory_controls_frame, text="Double-click a product to adjust inventory"
).pack(side=tk.LEFT, padx=5)
tk.Button(
    inventory_controls_frame,
    text="Refresh Inventory",
    command=lambda: load_inventory_status(),
).pack(side=tk.LEFT, padx=5)

# Inventory display
inventory_tree = ttk.Treeview(
    inventory_frame,
    columns=(
        "sku",
        "name",
        "stock",
        "reorder",
        "cost",
        "price",
        "value",
        "margin",
        "status",
    ),
    show="headings",
    height=15,
)

# Configure columns
inventory_tree.heading("sku", text="SKU")
inventory_tree.heading("name", text="Product Name")
inventory_tree.heading("stock", text="Stock")
inventory_tree.heading("reorder", text="Reorder Point")
inventory_tree.heading("cost", text="Unit Cost")
inventory_tree.heading("price", text="Selling Price")
inventory_tree.heading("value", text="Total Value")
inventory_tree.heading("margin", text="Profit %")
inventory_tree.heading("status", text="Status")

# Set column widths
inventory_tree.column("sku", width=80)
inventory_tree.column("name", width=200)
inventory_tree.column("stock", width=60)
inventory_tree.column("reorder", width=80)
inventory_tree.column("cost", width=80)
inventory_tree.column("price", width=80)
inventory_tree.column("value", width=80)
inventory_tree.column("margin", width=70)
inventory_tree.column("status", width=100)

# Bind double-click to inventory adjustment
inventory_tree.bind("<Double-1>", lambda e: adjust_inventory_dialog())

inventory_tree.pack(fill="both", expand=True, padx=5, pady=5)

# Inventory summary
summary_frame = tk.LabelFrame(inventory_tab, text="Summary", padx=10, pady=10)
summary_frame.pack(fill="x", padx=10, pady=5)

summary_text = tk.Text(summary_frame, height=3, wrap=tk.WORD)
summary_text.pack(fill="x")
summary_text.insert(tk.END, "Click 'Refresh Inventory' to load current stock levels.")
summary_text.config(state=tk.DISABLED)

# Load initial data
load_categories()
load_all_tags_for_list()
load_inventory_status()

root.mainloop()
