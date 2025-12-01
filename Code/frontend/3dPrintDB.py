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


# FastAPI endpoints
API_URL = "http://localhost:8000/products/"
TAGS_URL = "http://localhost:8000/tags"
TAGS_SUGGEST_URL = "http://localhost:8000/tags/suggest"
SEARCH_URL = "http://localhost:8000/products/search"
CATEGORIES_URL = "http://localhost:8000/categories"
INVENTORY_URL = "http://localhost:8000/inventory/status"

from modules.api_client import *
from modules import search
from modules.toggles import create_production_active_group, create_search_filter_group
from modules.ui_components import CheckRating, ErrorDialog


# Global flag to prevent multiple dialogs
dialog_open = False



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
