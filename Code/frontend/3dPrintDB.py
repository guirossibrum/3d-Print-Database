#!/usr/bin/env python3

import tkinter as tk
from tkinter import ttk, scrolledtext
from tkinter import messagebox
from tkinter import filedialog
import requests
import json
import os
import sys
import subprocess
import time
from pathlib import Path


# Ensure modules can be imported regardless of current directory
script_dir = os.path.dirname(os.path.abspath(__file__))
if script_dir not in sys.path:
    sys.path.insert(0, script_dir)


def show_copyable_error(title, message):
    """Show error dialog with copyable text using Text widget"""
    dialog = tk.Toplevel(root)
    dialog.title(title)
    dialog.geometry("500x300")

    # Error icon and title
    header_frame = tk.Frame(dialog)
    header_frame.pack(pady=10, padx=10, fill="x")

    # Simple error icon using text
    tk.Label(header_frame, text="⚠", font=("Arial", 24), fg="red").pack(
        side=tk.LEFT, padx=5
    )
    tk.Label(header_frame, text=title, font=("Arial", 14, "bold")).pack(
        side=tk.LEFT, padx=10
    )

    # Text widget for copyable message
    text_frame = tk.Frame(dialog)
    text_frame.pack(fill="both", expand=True, padx=10, pady=(0, 10))

    text_widget = tk.Text(text_frame, wrap=tk.WORD, height=10, padx=5, pady=5)
    scrollbar = tk.Scrollbar(text_frame, command=text_widget.yview)
    text_widget.config(yscrollcommand=scrollbar.set)

    text_widget.pack(side=tk.LEFT, fill="both", expand=True)
    scrollbar.pack(side=tk.RIGHT, fill=tk.Y)

    text_widget.insert(tk.END, message)
    text_widget.config(state=tk.DISABLED)  # Make read-only but selectable

    # Button frame
    button_frame = tk.Frame(dialog)
    button_frame.pack(pady=10)

    def copy_to_clipboard():
        """Copy the error message to clipboard"""
        root.clipboard_clear()
        root.clipboard_append(message)
        # Optional: show brief feedback
        copy_btn.config(text="Copied!")
        dialog.after(1000, lambda: copy_btn.config(text="Copy"))

    copy_btn = tk.Button(button_frame, text="Copy", command=copy_to_clipboard)
    copy_btn.pack(side=tk.LEFT, padx=5)

    tk.Button(button_frame, text="OK", command=dialog.destroy).pack(
        side=tk.LEFT, padx=5
    )

    # Make dialog modal
    dialog.transient(root)
    dialog.grab_set()
    root.wait_window(dialog)


def add_copy_menu_to_entry(entry_widget):
    """Add right-click context menu and keyboard shortcuts to Entry widget for copying/pasting text"""
    menu = tk.Menu(entry_widget, tearoff=0)
    menu.add_command(
        label="Copy (Ctrl+C)", command=lambda: copy_entry_text(entry_widget)
    )
    menu.add_command(
        label="Paste (Ctrl+V)", command=lambda: paste_to_entry(entry_widget)
    )

    def show_menu(event):
        menu.post(event.x_root, event.y_root)

    entry_widget.bind("<Button-3>", show_menu)  # Right-click
    entry_widget.bind("<Control-c>", lambda e: copy_entry_text(entry_widget))  # Ctrl+C
    entry_widget.bind("<Control-v>", lambda e: paste_to_entry(entry_widget))  # Ctrl+V


def copy_entry_text(entry_widget):
    """Copy the text from an Entry widget to clipboard"""
    text = entry_widget.get()
    if text:
        root.clipboard_clear()
        root.clipboard_append(text)


def paste_to_entry(entry_widget):
    """Paste text from clipboard to Entry widget"""
    try:
        text = root.clipboard_get()
        if text:
            # Clear current selection and insert clipboard content
            entry_widget.delete(0, tk.END)
            entry_widget.insert(0, text)
    except tk.TclError:
        # Clipboard empty or unavailable
        pass


# FastAPI endpoints
API_URL = "http://localhost:8000/products/"
TAGS_URL = "http://localhost:8000/tags"
TAGS_SUGGEST_URL = "http://localhost:8000/tags/suggest"
SEARCH_URL = "http://localhost:8000/products/search"
CATEGORIES_URL = "http://localhost:8000/categories"
INVENTORY_URL = "http://localhost:8000/inventory/status"

from modules.api_client import *
from modules import search
from modules.toggles import create_production_active_group
from modules.ui_components import CheckRating


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
            tag_frame.grid(row=i % 5, column=(i // 5) * 2, padx=2, pady=2, sticky="w")

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


def update_material_display(materials_list, display_frame, layout="pack"):
    """Update the display of materials with configurable layout"""
    # Clear existing
    for widget in display_frame.winfo_children():
        widget.destroy()

    if not materials_list:
        label = tk.Label(display_frame, text="(no materials)", fg="gray")
        if layout == "pack":
            label.pack(anchor="w")
        else:
            label.grid(row=0, column=0, sticky="w")
        return

    bg_color = "lightblue" if layout == "pack" else "lightgreen"

    for i, material in enumerate(materials_list):
        material_frame = tk.Frame(display_frame)

        if layout == "pack":
            material_frame.pack(anchor="w", pady=1)
        else:
            material_frame.grid(
                row=i % 5, column=(i // 5) * 2, padx=2, pady=2, sticky="w"
            )

        tk.Label(material_frame, text=material, bg=bg_color, padx=5, pady=2).pack(
            side=tk.LEFT
        )

        remove_btn = tk.Button(
            material_frame,
            text="×",
            font=("Arial", 8),
            command=lambda m=material: remove_popup_tag(
                m, materials_list, display_frame
            )
            if layout == "grid"
            else remove_material(m),
        )
        remove_btn.pack(side=tk.LEFT)


def add_popup_tag(widget, tags_list, display_frame, listbox=None, item_type="tag"):
    """Add a tag/material to the popup dialog"""
    item_text = widget.get().strip()
    if item_text and item_text not in tags_list:
        # Check if it exists in available items, if not, create it
        global all_available_tags, all_available_materials
        if item_type == "tag":
            available_items = all_available_tags
            create_func = create_tag
        else:  # material
            available_items = all_available_materials
            create_func = create_material

        # Check if item exists
        existing = next(
            (item for item in available_items if item["name"] == item_text), None
        )
        if not existing:
            try:
                # Create new item in DB
                new_item = create_func(item_text)
                available_items.append(new_item)
                available_items.sort(key=lambda x: x["name"])
                # Update listbox if provided
                if listbox:
                    listbox.delete(0, tk.END)
                    for item in available_items:
                        listbox.insert(tk.END, item["name"])
            except Exception as e:
                show_copyable_error("Error", f"Failed to create {item_type}: {str(e)}")
                return

        tags_list.append(item_text)
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


# Global variables
current_tags = []
current_materials = []
tag_suggestions = []
all_available_tags = []  # All existing tags for the list
inventory_sort_orders = {}  # Track sort order for inventory columns
all_available_materials = []  # All existing materials for the list
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
    var_production.set(False)
    rating_widget.set_rating_direct(0)
    current_tags.clear()
    update_tag_display(current_tags, tags_frame, "grid")
    tag_entry.delete(0, tk.END)
    current_materials.clear()
    update_material_display(current_materials, materials_frame, "grid")
    material_entry.delete(0, tk.END)
    # Reset category to first if available
    if categories:
        category_combo.current(0)
        selected_category_id = categories[0]["id"]
    else:
        category_combo.set("")


def cancel():
    root.destroy()


def add_tag():
    """Add one or more tags to the current tags list (comma-separated)"""
    tag_text = tag_entry.get().strip()
    if tag_text:
        # Split on commas and process each tag
        tag_entries = [tag.strip() for tag in tag_text.split(",") if tag.strip()]
        added_count = 0
        for tag in tag_entries:
            if tag and tag not in current_tags:
                current_tags.append(tag)
                added_count += 1

        if added_count > 0:
            update_tag_display(current_tags, tags_frame, "grid")
            tag_entry.delete(0, tk.END)  # Clear the input
            tag_entry.focus()


def remove_tag(tag_to_remove):
    """Remove a tag from the current tags list"""
    if tag_to_remove in current_tags:
        current_tags.remove(tag_to_remove)
        update_tag_display(current_tags, tags_frame, "grid")


def add_material():
    """Add one or more materials to the current materials list (comma-separated)"""
    material_text = material_entry.get().strip()
    if material_text:
        # Split on commas and process each material
        material_entries = [
            material.strip()
            for material in material_text.split(",")
            if material.strip()
        ]
        added_count = 0
        for material in material_entries:
            if material and material not in current_materials:
                current_materials.append(material)
                added_count += 1

        if added_count > 0:
            update_material_display(current_materials, materials_frame, "grid")
            material_entry.delete(0, tk.END)  # Clear the input
            material_entry.focus()


def remove_material(material_to_remove):
    """Remove a material from the current materials list"""
    if material_to_remove in current_materials:
        current_materials.remove(material_to_remove)
        update_material_display(current_materials, materials_frame, "grid")


# Removed autocomplete functions - using list-based tag selection now


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
            show_copyable_error(
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
            show_copyable_error(
                "Materials Error",
                f"Failed to load materials: {response.status_code} - {response.text[:200]}",
            )
    except Exception as e:
        show_copyable_error("Materials Error", f"Error loading materials: {str(e)}")


def get_tag_ids_from_names(tag_names):
    """Convert tag names to tag IDs"""
    return [
        tag["id"]
        for tag in all_available_tags
        if tag["name"] in tag_names and tag["id"] is not None
    ]


def get_material_ids_from_names(material_names):
    """Convert material names to material IDs"""
    return [
        mat["id"]
        for mat in all_available_materials
        if mat["name"] in material_names and mat["id"] is not None
    ]


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


def add_tag_from_list(event=None):
    """Add selected tag from the listbox"""
    add_tag_from_listbox(
        tag_listbox,
        current_tags,
        lambda tags: update_tag_display(tags, tags_frame, "grid"),
    )
    tag_entry.delete(0, tk.END)  # Clear input
    tag_entry.focus()


def add_material_from_list(event=None):
    """Add selected material from the listbox"""
    add_tag_from_listbox(
        material_listbox,
        current_materials,
        lambda materials: update_material_display(materials, materials_frame, "grid"),
    )
    material_entry.delete(0, tk.END)  # Clear input
    material_entry.focus()


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


def build_product_payload(
    name,
    description,
    production,
    active,
    category_id,
    rating,
    tag_names,
    material_names,
    **extra_fields,
):
    """Build product payload for create/update operations"""
    payload = {
        "name": name,
        "description": description,
        "tag_ids": get_tag_ids_from_names(tag_names),
        "material_ids": get_material_ids_from_names(material_names),
        "production": production,
        "active": active,
        "category_id": category_id,
        "rating": rating,
    }
    payload.update(extra_fields)  # Add any additional fields
    return payload


def create_item():
    name = entry_name.get().strip()
    description = entry_description.get().strip()
    production = var_production.get()
    active = var_active.get()

    if not name:
        show_copyable_error("Error", "Name is required")
        return

    if not selected_category_id:
        show_copyable_error("Error", "Please select a category")
        return

    # Build JSON payload using shared function
    payload = build_product_payload(
        name=name,
        description=description,
        production=production,
        active=active,
        category_id=selected_category_id,
        rating=rating_widget.get_rating(),
        tag_names=current_tags,
        material_names=current_materials,
    )

    try:
        response = requests.post(API_URL, json=payload)
        if response.status_code == 200:
            messagebox.showinfo(
                "Success", f"Product created: {response.json().get('sku')}"
            )
            update_available_tags(current_tags)
            clear_form()
        else:
            show_copyable_error("Error", f"Failed to create product\n{response.text}")
    except Exception as e:
        show_copyable_error("Error", str(e))


# --- Update/Search Functions ---


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
            show_copyable_error("Error", f"Failed to load categories: {response.text}")
    except Exception as e:
        show_copyable_error("Error", f"Error loading categories: {str(e)}")


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
    add_copy_menu_to_entry(name_entry)

    tk.Label(dialog, text="SKU Initials (3 letters):").grid(
        row=1, column=0, sticky="e", padx=5, pady=5
    )
    initials_entry = tk.Entry(dialog, width=10)
    initials_entry.grid(row=1, column=1, sticky="w", padx=5, pady=5)
    add_copy_menu_to_entry(initials_entry)

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
            show_copyable_error("Error", "Name and SKU initials are required")
            return

        if len(initials) != 3 or not initials.isalnum():
            show_copyable_error(
                "Error", "SKU initials must be exactly 3 alphanumeric characters"
            )
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
            show_copyable_error("Error", f"Error creating category: {str(e)}")

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
        show_copyable_error("Error", "Category not found")
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
    add_copy_menu_to_entry(name_entry)

    tk.Label(dialog, text="SKU Initials (3 letters):").grid(
        row=1, column=0, sticky="e", padx=5, pady=5
    )
    initials_entry = tk.Entry(dialog, width=10)
    initials_entry.insert(0, category["sku_initials"])
    initials_entry.grid(row=1, column=1, sticky="w", padx=5, pady=5)
    add_copy_menu_to_entry(initials_entry)

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
            show_copyable_error("Error", "Name and SKU initials are required")
            return

        if len(initials) != 3 or not initials.isalnum():
            show_copyable_error(
                "Error", "SKU initials must be exactly 3 alphanumeric characters"
            )
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
            show_copyable_error("Error", f"Error updating category: {str(e)}")

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
        show_copyable_error("Error", "Category not found")
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
            show_copyable_error("Error", f"Failed to delete category: {response.text}")
    except Exception as e:
        show_copyable_error("Error", f"Error deleting category: {str(e)}")


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


# Global flag to prevent multiple dialogs
dialog_open = False


def do_search():
    global \
        var_include_inactive, \
        var_include_prototype, \
        search_query, \
        results_text, \
        search_results
    search.search_products(
        search_query,
        results_text,
        search_results,
        var_include_inactive.get(),
        var_include_prototype.get(),
    )


def load_product_from_search():
    """Load product from search results for editing (double-click)"""
    search.load_product_from_search(results_text, search_results, show_edit_callback)


def add_tag_from_listbox(listbox, current_tags, update_func):
    """Generic helper to add tag from listbox"""
    selection = listbox.curselection()
    if selection:
        tag = listbox.get(selection[0])
        if tag not in current_tags:
            current_tags.append(tag)
            update_func(current_tags)


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


def show_edit_product_dialog(product):
    """Show popup dialog for editing a product"""
    global edit_current_tags, edit_current_materials, current_product_data, edit_mode

    # Refresh available tags and materials from database
    load_all_tags_for_list()
    load_all_materials_for_list()

    # Set global state
    current_product_data = product
    edit_mode = True
    # Handle tags as list of strings or dicts
    tags_list = product.get("tags", [])
    if tags_list and isinstance(tags_list[0], dict):
        edit_current_tags = [tag["name"] for tag in tags_list]
    else:
        edit_current_tags = tags_list.copy()

    # Handle materials
    materials_list = product.get("materials", [])
    if materials_list and isinstance(materials_list[0], dict):
        edit_current_materials = [m["name"] for m in materials_list]
    else:
        edit_current_materials = materials_list.copy()

    # Create edit dialog
    dialog = tk.Toplevel(root)
    dialog.title(f"Edit Product - {product['id']}")
    dialog.geometry("800x700")

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
    add_copy_menu_to_entry(edit_name)

    # Description
    tk.Label(main_frame, text="Description:").grid(
        row=1, column=0, sticky="e", padx=5, pady=5
    )
    edit_description = tk.Entry(main_frame, width=50)
    desc_value = product.get("description")
    if desc_value is not None:
        edit_description.insert(0, str(desc_value))
    edit_description.grid(row=1, column=1, columnspan=3, pady=5, padx=5, sticky="w")
    add_copy_menu_to_entry(edit_description)

    # Production and Active checkboxes
    edit_var_production = tk.BooleanVar(value=product["production"])
    tk.Checkbutton(
        main_frame, text="Production Ready", variable=edit_var_production
    ).grid(row=2, column=1, sticky="w", pady=5, padx=5)

    edit_var_active = tk.BooleanVar(value=product["active"])
    tk.Checkbutton(main_frame, text="Active", variable=edit_var_active).grid(
        row=2, column=2, sticky="w", pady=5, padx=5
    )

    # Rating
    tk.Label(main_frame, text="Rating:").grid(
        row=4, column=0, sticky="e", pady=5, padx=5
    )
    edit_rating_widget = CheckRating(
        main_frame, initial_rating=product.get("rating", 0)
    )
    edit_rating_widget.grid(row=4, column=1, sticky="w", pady=5, padx=5)

    # Color
    tk.Label(main_frame, text="Color:").grid(
        row=4, column=2, sticky="e", padx=5, pady=2
    )
    edit_color = tk.Entry(main_frame, width=20)
    color_value = product.get("color")
    if color_value is not None:
        edit_color.insert(0, str(color_value))
    edit_color.grid(row=4, column=3, pady=2, padx=5, sticky="w")
    add_copy_menu_to_entry(edit_color)

    # Print time and Weight
    tk.Label(main_frame, text="Print Time:").grid(
        row=5, column=0, sticky="e", padx=5, pady=2
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
    edit_print_time.grid(row=5, column=1, pady=2, padx=5, sticky="w")
    add_copy_menu_to_entry(edit_print_time)

    tk.Label(main_frame, text="Weight (g):").grid(
        row=5, column=2, sticky="e", padx=5, pady=2
    )
    edit_weight = tk.Entry(main_frame, width=20)
    weight_value = product.get("weight")
    if weight_value is not None:
        edit_weight.insert(0, str(weight_value))
    edit_weight.grid(row=5, column=3, pady=2, padx=5, sticky="w")
    add_copy_menu_to_entry(edit_weight)

    # Tags section
    tk.Label(main_frame, text="Tags:").grid(
        row=6, column=0, sticky="ne", pady=5, padx=5
    )

    edit_tag_frame = tk.Frame(main_frame)
    edit_tag_frame.grid(row=6, column=1, columnspan=3, pady=5, padx=5, sticky="w")

    edit_tag_entry = tk.Entry(edit_tag_frame, width=30)
    edit_tag_entry.pack(side=tk.LEFT, padx=(0, 5))
    add_copy_menu_to_entry(edit_tag_entry)

    edit_add_btn = tk.Button(
        edit_tag_frame,
        text="Add Tag(s)",
        command=lambda: add_popup_tag(
            edit_tag_entry, edit_current_tags, edit_tags_frame, edit_tag_listbox, "tag"
        ),
    )
    edit_add_btn.pack(side=tk.LEFT)

    # Available tags list
    tk.Label(main_frame, text="Available Tags:").grid(
        row=7, column=0, sticky="ne", pady=5, padx=5
    )

    edit_tag_list_frame = tk.Frame(main_frame)
    edit_tag_list_frame.grid(row=7, column=1, pady=5, padx=5, sticky="nw")

    tk.Label(main_frame, text="Selected Tags:").grid(
        row=7, column=2, sticky="ne", pady=5, padx=5
    )
    edit_tags_frame = tk.Frame(main_frame)
    edit_tags_frame.grid(row=7, column=3, pady=5, padx=5, sticky="nw")

    # Initialize tag display
    update_tag_display(edit_current_tags, edit_tags_frame, "grid")

    edit_tag_listbox = tk.Listbox(edit_tag_list_frame, height=10, width=25)
    edit_tag_listbox.pack(side=tk.LEFT, fill=tk.BOTH, expand=True)

    edit_tag_scrollbar = tk.Scrollbar(edit_tag_list_frame)
    edit_tag_scrollbar.pack(side=tk.RIGHT, fill=tk.Y)

    edit_tag_listbox.config(yscrollcommand=edit_tag_scrollbar.set)
    edit_tag_scrollbar.config(command=edit_tag_listbox.yview)

    # Populate listbox
    for tag in all_available_tags:
        edit_tag_listbox.insert(tk.END, tag["name"])

    # Bind double-click to add
    edit_tag_listbox.bind(
        "<Double-1>",
        lambda e: add_tag_from_listbox(
            edit_tag_listbox,
            edit_current_tags,
            lambda tags: update_tag_display(tags, edit_tags_frame, "grid"),
        ),
    )

    # Materials section
    tk.Label(main_frame, text="Materials:").grid(
        row=9, column=0, sticky="ne", pady=5, padx=5
    )

    edit_material_frame = tk.Frame(main_frame)
    edit_material_frame.grid(row=9, column=1, columnspan=3, pady=5, padx=5, sticky="w")

    edit_material_entry = tk.Entry(edit_material_frame, width=30)
    edit_material_entry.pack(side=tk.LEFT, padx=(0, 5))
    add_copy_menu_to_entry(edit_material_entry)

    edit_add_material_btn = tk.Button(
        edit_material_frame,
        text="Add Material(s)",
        command=lambda: add_popup_tag(
            edit_material_entry,
            edit_current_materials,
            edit_materials_frame,
            edit_material_listbox,
            "material",
        ),
    )
    edit_add_material_btn.pack(side=tk.LEFT)

    # Available materials list
    tk.Label(main_frame, text="Available Materials:").grid(
        row=10, column=0, sticky="ne", pady=5, padx=5
    )

    edit_material_list_frame = tk.Frame(main_frame)
    edit_material_list_frame.grid(row=10, column=1, pady=5, padx=5, sticky="nw")

    tk.Label(main_frame, text="Selected Materials:").grid(
        row=10, column=2, sticky="ne", pady=5, padx=5
    )
    edit_materials_frame = tk.Frame(main_frame)
    edit_materials_frame.grid(row=10, column=3, pady=5, padx=5, sticky="nw")

    # Initialize material display
    update_material_display(edit_current_materials, edit_materials_frame, "grid")

    edit_material_listbox = tk.Listbox(edit_material_list_frame, height=10, width=25)
    edit_material_listbox.pack(side=tk.LEFT, fill=tk.BOTH, expand=True)

    edit_material_scrollbar = tk.Scrollbar(edit_material_list_frame)
    edit_material_scrollbar.pack(side=tk.RIGHT, fill=tk.Y)

    edit_material_listbox.config(yscrollcommand=edit_material_scrollbar.set)
    edit_material_scrollbar.config(command=edit_material_listbox.yview)

    # Populate listbox
    for m in all_available_materials:
        edit_material_listbox.insert(tk.END, m["name"])

    # Bind double-click to add
    edit_material_listbox.bind(
        "<Double-1>",
        lambda e: add_tag_from_listbox(
            edit_material_listbox,
            edit_current_materials,
            lambda mats: update_tag_display(mats, edit_materials_frame, "grid"),
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
            active = edit_var_active.get()
            color = edit_color.get().strip()
            print_time = edit_print_time.get().strip()
            weight_text = edit_weight.get().strip()

            if not name:
                show_copyable_error("Error", "Name is required")
                return

            # Build payload using shared function
            payload = build_product_payload(
                name=name,
                description=description,
                production=production,
                active=active,
                category_id=None,  # Not changing category in edit mode
                rating=edit_rating_widget.get_rating(),
                tag_names=edit_current_tags,
                material_names=edit_current_materials,
                color=color or None,
                print_time=print_time or None,
                weight=int(weight_text) if weight_text else None,
                product_id=product["id"],  # Include product_id for update
            )

            # Update product
            save_product_changes(product["id"], payload)
            # Add new tags to available tags (not saved to DB)
            original_tags = product.get("tags", [])
            new_tags = [t for t in edit_current_tags if t not in original_tags]
            update_available_tags(new_tags)
            global dialog_open
            dialog_open = False
            dialog.destroy()
            # Refresh search results
            do_search()

        except Exception as e:
            show_copyable_error("Error", f"Error updating product: {str(e)}")

    def open_folder():
        """Open the product folder"""
        try:
            import os

            # Use stored folder path first
            folder_path = product.get("folder_path")

            # If stored path doesn't exist, try to construct the expected path
            if not folder_path or not os.path.exists(folder_path):
                # Try the new naming scheme: SKU - Name
                products_dir = os.environ.get(
                    "PRODUCTS_DIR",
                    os.path.join(
                        os.path.expanduser("~"), "Work", "3d_print", "Products"
                    ),
                )
                sku = product.get("sku", "")
                name = product.get("name", "")
                expected_path = os.path.join(products_dir, f"{sku} - {name}")

                if os.path.exists(expected_path):
                    folder_path = expected_path
                else:
                    # Try old naming scheme: just SKU
                    old_path = os.path.join(products_dir, sku)
                    if os.path.exists(old_path):
                        folder_path = old_path
                    else:
                        show_copyable_error(
                            "Folder Not Found",
                            f"The folder for product '{sku}' does not exist at any expected location.",
                        )
                        return

            if not os.path.exists(folder_path):
                show_copyable_error(
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
            show_copyable_error("Error", f"Could not open folder: {str(e)}")

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
                f"{API_URL}{product['id']}?delete_files={delete_files}"
            )
            if response.status_code == 200:
                messagebox.showinfo(
                    "Success",
                    f"Product {product['sku']} ({product['name']}) deleted successfully!",
                )
                global dialog_open
                dialog_open = False
                dialog.destroy()
                # Refresh search results
                do_search()
            else:
                show_copyable_error(
                    "Error", f"Failed to delete product: {response.text}"
                )

        except Exception as e:
            show_copyable_error("Error", f"Error deleting product: {str(e)}")

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
    global inventory_tree
    selected_item = inventory_tree.selection()
    if not selected_item:
        messagebox.showwarning(
            "No Selection", "Please double-click on a product to adjust inventory."
        )
        return

    # Get selected product data
    item_values = inventory_tree.item(selected_item[0], "values")
    tags = inventory_tree.item(selected_item[0], "tags")
    product_id = int(tags[0])
    sku = item_values[0]
    product_name = item_values[1]
    current_stock = int(item_values[2]) if item_values[2].isdigit() else 0
    current_reorder = int(item_values[3]) if item_values[3].isdigit() else 0

    # Create simple adjustment dialog
    dialog = tk.Toplevel(root)
    dialog.title(f"Adjust Inventory - {sku}")
    dialog.geometry("400x300")

    # Product info
    tk.Label(dialog, text=f"Product: {product_name}", font=("Arial", 10, "bold")).pack(
        pady=10
    )
    tk.Label(dialog, text=f"Current Stock: {current_stock}").pack(pady=5)
    tk.Label(dialog, text=f"Current Reorder Point: {current_reorder}").pack(pady=5)

    # Quantity input
    tk.Label(dialog, text="Quantity:").pack(pady=5)
    quantity_entry = tk.Entry(dialog, width=10, justify="center")
    quantity_entry.pack(pady=5)
    quantity_entry.focus()
    add_copy_menu_to_entry(quantity_entry)

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

    # Reorder point input
    tk.Label(dialog, text="New Reorder Point:").pack(pady=5)
    reorder_entry = tk.Entry(dialog, width=10, justify="center")
    reorder_entry.insert(0, str(current_reorder) if current_reorder != 0 else "")
    reorder_entry.pack(pady=5)
    add_copy_menu_to_entry(reorder_entry)

    def adjust_stock():
        """Adjust stock based on selected operation"""
        try:
            quantity_str = quantity_entry.get().strip()
            quantity = int(quantity_str) if quantity_str else 0
            if quantity < 0:
                raise ValueError("Quantity cannot be negative")

            operation = operation_var.get() if quantity > 0 else None
            reorder_str = reorder_entry.get().strip()
            new_reorder = int(reorder_str) if reorder_str else 0
            reorder_point = new_reorder if new_reorder != current_reorder else None

            # Apply adjustment
            success_message = apply_inventory_adjustment(
                sku, product_id, operation, quantity, current_stock, reorder_point
            )

            messagebox.showinfo("Success", success_message)
            global dialog_open
            dialog_open = False
            dialog.destroy()
            load_inventory_status()  # Refresh inventory display

        except ValueError as e:
            show_copyable_error("Invalid Input", str(e))
        except Exception as e:
            show_copyable_error("Error", f"Error updating inventory: {str(e)}")

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
root.attributes("-topmost", True)  # Make window always on top

# Tkinter variables
include_out_of_stock_var = tk.BooleanVar(value=False)
need_to_produce_var = tk.BooleanVar(value=False)
var_include_inactive = tk.BooleanVar(value=False)
var_include_prototype = tk.BooleanVar(value=False)

tab_control = ttk.Notebook(root)

# Create tab frames
create_tab = ttk.Frame(tab_control)
update_tab = ttk.Frame(tab_control)
inventory_tab = ttk.Frame(tab_control)

tab_control.add(create_tab, text="Create Product")
tab_control.add(update_tab, text="Search")
tab_control.add(inventory_tab, text="Inventory")

# Make Search tab the default
tab_control.select(update_tab)


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
        # Load all existing tags and materials for the list when Create Product tab is selected
        load_all_tags_for_list()
        load_all_materials_for_list()
    elif tab_text == "Search":
        # Auto-load all products when Search tab is selected
        search_query.delete(0, tk.END)  # Clear search field
        do_search()  # Load all products
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
add_copy_menu_to_entry(entry_name)

# Description field (longer field)
tk.Label(create_tab, text="Description:").grid(
    row=1, column=0, sticky="e", padx=5, pady=5
)
entry_description = tk.Entry(create_tab, width=50)
entry_description.grid(row=1, column=1, columnspan=3, pady=5, padx=5, sticky="w")
add_copy_menu_to_entry(entry_description)

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

# Production and Active toggles using new modular approach
from modules.toggles import create_production_active_group

production_active_group = create_production_active_group(create_tab, start_row=3)
production_toggles = production_active_group.get_values()

# Search filter variables
var_include_inactive = tk.BooleanVar(value=False)
var_include_prototype = tk.BooleanVar(value=False)

# Rating section
tk.Label(create_tab, text="Rating:").grid(row=4, column=0, sticky="e", pady=5, padx=5)
rating_widget = CheckRating(create_tab, initial_rating=0)
rating_widget.grid(row=4, column=1, sticky="w", pady=5, padx=5)

# Tags section
tk.Label(create_tab, text="Tags:", font=("Arial", 10, "bold")).grid(
    row=5, column=0, sticky="e", pady=5, padx=5
)

# Tag input frame (same row as Available Tags)
tag_input_frame = tk.Frame(create_tab)
tag_input_frame.grid(row=5, column=1, columnspan=2, pady=5, padx=5, sticky="w")

# Tag input entry (simple text field)
tag_entry = tk.Entry(tag_input_frame, width=30)
tag_entry.pack(side=tk.LEFT, padx=(0, 10))
tag_entry.bind("<KeyRelease>", filter_tag_list)
add_copy_menu_to_entry(tag_entry)

# Add tag button
add_btn = tk.Button(tag_input_frame, text="Add Tag(s)", command=add_tag)
add_btn.pack(side=tk.LEFT)

# Available tags list
tk.Label(create_tab, text="Available Tags:").grid(
    row=6, column=0, sticky="ne", pady=5, padx=5
)
tag_list_frame = tk.Frame(create_tab)
tag_list_frame.grid(row=6, column=1, pady=5, padx=5, sticky="nw")


tag_listbox = tk.Listbox(tag_list_frame, width=25, height=10, selectmode=tk.SINGLE)
tag_listbox.pack(fill=tk.BOTH, expand=True)
tag_listbox.bind("<Double-1>", add_tag_from_list)

# Delete tag button
delete_tag_btn = tk.Button(tag_list_frame, text="Delete Tag", command=delete_unused_tag)
delete_tag_btn.pack(pady=(5, 0))

# Current tags display frame
tk.Label(create_tab, text="Selected Tags:").grid(
    row=6, column=2, sticky="ne", pady=5, padx=5
)
tags_frame = tk.Frame(create_tab)
tags_frame.grid(row=6, column=3, pady=5, padx=5, sticky="nw")

# Materials section
tk.Label(create_tab, text="Materials:", font=("Arial", 10, "bold")).grid(
    row=8, column=0, sticky="e", pady=5, padx=5
)

# Material input frame (same row as Available Materials)
material_input_frame = tk.Frame(create_tab)
material_input_frame.grid(row=8, column=1, columnspan=2, pady=5, padx=5, sticky="w")

# Material input entry (simple text field)
material_entry = tk.Entry(material_input_frame, width=30)
material_entry.pack(side=tk.LEFT, padx=(0, 10))
material_entry.bind("<KeyRelease>", lambda e: filter_material_list())
add_copy_menu_to_entry(material_entry)

# Add material button
add_material_btn = tk.Button(
    material_input_frame, text="Add Material(s)", command=add_material
)
add_material_btn.pack(side=tk.LEFT)

# Available materials list
tk.Label(create_tab, text="Available Materials:").grid(
    row=9, column=0, sticky="ne", pady=5, padx=5
)
material_list_frame = tk.Frame(create_tab)
material_list_frame.grid(row=9, column=1, pady=5, padx=5, sticky="nw")


material_listbox = tk.Listbox(
    material_list_frame, width=25, height=10, selectmode=tk.SINGLE
)
material_listbox.pack(fill=tk.BOTH, expand=True)
material_listbox.bind("<Double-1>", add_material_from_list)

# Delete material button
delete_material_btn = tk.Button(
    material_list_frame, text="Delete Material", command=delete_unused_material
)
delete_material_btn.pack(pady=(5, 0))

# Current materials display frame
tk.Label(create_tab, text="Selected Materials:").grid(
    row=9, column=2, sticky="ne", pady=5, padx=5
)
materials_frame = tk.Frame(create_tab)
materials_frame.grid(row=9, column=3, pady=5, padx=5, sticky="nw")

# Create tab buttons
create_button_frame = tk.Frame(create_tab)
create_button_frame.grid(row=10, column=0, columnspan=6, pady=15)

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
search_query.bind("<KeyRelease>", lambda e: do_search())  # Active filtering
add_copy_menu_to_entry(search_query)
tk.Label(search_frame, text="(searches name, SKU, and tags)").grid(
    row=0, column=2, padx=5, pady=2
)

tk.Button(search_frame, text="Search", command=do_search).grid(
    row=0, column=6, padx=10, pady=2
)

# Filter checkboxes
tk.Checkbutton(
    search_frame,
    text="Include Inactive",
    variable=var_include_inactive,
    command=do_search,
).grid(row=1, column=1, sticky="w", padx=5, pady=2)
tk.Checkbutton(
    search_frame,
    text="Include Prototype",
    variable=var_include_prototype,
    command=do_search,
).grid(row=1, column=2, sticky="w", padx=5, pady=2)

# Results section
results_frame = tk.LabelFrame(update_tab, text="Search Results", padx=10, pady=10)
results_frame.pack(fill="both", expand=True, padx=10, pady=5)

results_text = scrolledtext.ScrolledText(results_frame, height=8, wrap=tk.WORD)
results_text.pack(fill="both", expand=True)

# Bind double-click to load product for editing
results_text.bind("<Double-1>", lambda e: load_product_from_search())

# Inventory controls
inventory_controls_frame = tk.Frame(inventory_tab)
inventory_controls_frame.pack(pady=5)

tk.Label(
    inventory_controls_frame, text="Double-click a product to adjust inventory"
).pack(side=tk.LEFT, padx=5)
tk.Button(
    inventory_controls_frame,
    text="Refresh Inventory",
    command=lambda: load_inventory_status(),
).pack(side=tk.LEFT, padx=5)

# Filter checkboxes
tk.Checkbutton(
    inventory_controls_frame,
    text="Include out of stock items",
    variable=include_out_of_stock_var,
    command=lambda: load_inventory_status(),
).pack(side=tk.LEFT, padx=10)

tk.Checkbutton(
    inventory_controls_frame,
    text="Need to produce",
    variable=need_to_produce_var,
    command=lambda: load_inventory_status(),
).pack(side=tk.LEFT, padx=10)

# Inventory display
inventory_tree = ttk.Treeview(
    inventory_tab,
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
inventory_tree.heading("sku", text="SKU", command=lambda: sort_inventory_column("sku"))
inventory_tree.heading(
    "name", text="Product Name", command=lambda: sort_inventory_column("name")
)
inventory_tree.heading(
    "stock", text="Stock", command=lambda: sort_inventory_column("stock")
)
inventory_tree.heading(
    "reorder", text="Reorder Point", command=lambda: sort_inventory_column("reorder")
)
inventory_tree.heading(
    "cost", text="Unit Cost", command=lambda: sort_inventory_column("cost")
)
inventory_tree.heading(
    "price", text="Selling Price", command=lambda: sort_inventory_column("price")
)
inventory_tree.heading(
    "value", text="Total Value", command=lambda: sort_inventory_column("value")
)
inventory_tree.heading(
    "margin", text="Profit %", command=lambda: sort_inventory_column("margin")
)
inventory_tree.heading(
    "status", text="Status", command=lambda: sort_inventory_column("status")
)

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
