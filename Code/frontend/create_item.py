#!/usr/bin/env python3

import tkinter as tk
from tkinter import ttk, messagebox
import requests
import json
import threading
import time

# FastAPI endpoints
API_URL = "http://localhost:8000/products/"
TAGS_SUGGEST_URL = "http://localhost:8000/tags/suggest"

# Tag management
current_tags = []
tag_suggestions = []


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


def fetch_tag_suggestions(partial_tag):
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
                    root.after(0, lambda: update_combo_suggestions(suggestions))
        except:
            pass  # Ignore network errors

    # Run in background thread
    threading.Thread(target=_fetch, daemon=True).start()


def update_combo_suggestions(suggestions):
    """Update the combobox with suggestions"""
    global tag_suggestions
    tag_suggestions = suggestions
    tag_combo["values"] = tag_suggestions


def on_tag_typing(event):
    """Handle typing in the tag input field"""
    partial = tag_combo.get()
    if len(partial.strip()) >= 2:
        fetch_tag_suggestions(partial)


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


# --- GUI ---
root = tk.Tk()
root.title("Create Product")

# Name field
tk.Label(root, text="Name:").grid(row=0, column=0, sticky="e")
entry_name = tk.Entry(root, width=30)
entry_name.grid(row=0, column=1, columnspan=2, pady=5)

# Description field
tk.Label(root, text="Description:").grid(row=1, column=0, sticky="e")
entry_description = tk.Entry(root, width=30)
entry_description.grid(row=1, column=1, columnspan=2, pady=5)

# Production checkbox
var_production = tk.BooleanVar(value=True)
tk.Checkbutton(root, text="Production", variable=var_production).grid(
    row=2, column=1, sticky="w", pady=5
)

# Tags section
tk.Label(root, text="Tags:").grid(row=3, column=0, sticky="ne", pady=5)

# Tag input frame
tag_frame = tk.Frame(root)
tag_frame.grid(row=3, column=1, columnspan=2, pady=5, sticky="w")

# Tag input combobox with autocomplete
tag_combo = ttk.Combobox(tag_frame, width=25)
tag_combo.pack(side=tk.LEFT, padx=(0, 5))
tag_combo.bind("<KeyRelease>", on_tag_typing)
tag_combo.bind("<Return>", lambda e: add_tag())  # Enter key adds tag

# Add tag button
add_btn = tk.Button(tag_frame, text="Add Tag", command=add_tag)
add_btn.pack(side=tk.LEFT)

# Current tags display frame
tags_frame = tk.Frame(root)
tags_frame.grid(row=4, column=0, columnspan=3, pady=5, sticky="w")

# Buttons
button_frame = tk.Frame(root)
button_frame.grid(row=5, column=0, columnspan=3, pady=10)

tk.Button(button_frame, text="Clear", command=clear_form).pack(side=tk.LEFT, padx=5)
tk.Button(button_frame, text="Cancel", command=cancel).pack(side=tk.LEFT, padx=5)
tk.Button(button_frame, text="Create Item", command=create_item).pack(
    side=tk.LEFT, padx=5
)

root.mainloop()
