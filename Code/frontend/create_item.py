#!/home/grbrum/Work/3d_print/Code/.venv/bin/python

import tkinter as tk
from tkinter import messagebox
import requests
import json

# FastAPI endpoint
API_URL = "http://localhost:8000/products/"


def clear_form():
    entry_name.delete(0, tk.END)
    entry_description.delete(0, tk.END)
    var_production.set(True)


def cancel():
    root.destroy()


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
        "tags": [],  # optional, can be extended later
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

tk.Label(root, text="Name:").grid(row=0, column=0, sticky="e")
entry_name = tk.Entry(root, width=30)
entry_name.grid(row=0, column=1)

tk.Label(root, text="Description:").grid(row=1, column=0, sticky="e")
entry_description = tk.Entry(root, width=30)
entry_description.grid(row=1, column=1)

var_production = tk.BooleanVar(value=True)
tk.Checkbutton(root, text="Production", variable=var_production).grid(
    row=2, column=1, sticky="w"
)

# Buttons
tk.Button(root, text="Clear", command=clear_form).grid(row=3, column=0, pady=10)
tk.Button(root, text="Cancel", command=cancel).grid(row=3, column=1)
tk.Button(root, text="Create Item", command=create_item).grid(row=3, column=2, padx=5)

root.mainloop()
