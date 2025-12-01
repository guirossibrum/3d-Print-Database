# frontend/modules/multi_selection.py
"""Unified multi-selection widget for tags and materials"""

import tkinter as tk
from tkinter import messagebox
import requests
from .constants import TAGS_URL, MATERIALS_URL


class MultiSelectionWidget:
    """Unified widget for managing multi-selection of items (tags or materials)"""

    def __init__(self, parent, item_type="tag", callback=None):
        self.parent = parent
        self.item_type = item_type
        self.callback = callback
        self.current_items = []
        self.all_available_items = []

        # Create main frame
        self.frame = tk.Frame(parent)

        # Input section
        self.input_frame = tk.Frame(self.frame)
        self.input_frame.pack(fill="x", pady=5)

        tk.Label(self.input_frame, text=f"{item_type.title()}:").pack(
            side="left", padx=5
        )
        self.entry = tk.Entry(self.input_frame, width=30)
        self.entry.pack(side="left", padx=5)
        self.entry.bind("<Return>", lambda e: self.add_from_entry())

        self.add_btn = tk.Button(
            self.input_frame, text="Add", command=self.add_from_entry
        )
        self.add_btn.pack(side="left", padx=5)

        # Available items section
        self.available_frame = tk.Frame(self.frame)
        self.available_frame.pack(fill="both", expand=True, pady=5)

        tk.Label(self.available_frame, text=f"Available {item_type.title()}s:").pack(
            anchor="w"
        )

        # Filter entry
        self.filter_entry = tk.Entry(self.available_frame)
        self.filter_entry.pack(fill="x", pady=2)
        self.filter_entry.bind("<KeyRelease>", self.filter_list)

        # Listbox with scrollbar
        list_frame = tk.Frame(self.available_frame)
        list_frame.pack(fill="both", expand=True)

        scrollbar = tk.Scrollbar(list_frame)
        scrollbar.pack(side="right", fill="y")

        self.listbox = tk.Listbox(
            list_frame, yscrollcommand=scrollbar.set, selectmode="single"
        )
        self.listbox.pack(side="left", fill="both", expand=True)
        scrollbar.config(command=self.listbox.yview)

        self.listbox.bind("<Double-Button-1>", self.add_from_listbox)

        # Current items display
        self.current_frame = tk.Frame(self.frame)
        self.current_frame.pack(fill="x", pady=5)

        tk.Label(self.current_frame, text=f"Current {item_type.title()}s:").pack(
            anchor="w"
        )

        self.display_frame = tk.Frame(self.current_frame)
        self.display_frame.pack(fill="x", pady=2)

        # Delete button
        self.delete_btn = tk.Button(
            self.current_frame,
            text=f"Delete Unused {item_type.title()}",
            command=self.delete_unused_item,
        )
        self.delete_btn.pack(pady=5)

        # Load initial data
        self.load_all_items()

    def add_from_entry(self):
        """Add item from entry field"""
        item_text = self.entry.get().strip()
        if item_text and item_text not in self.current_items:
            # Check if item exists, create if not
            existing = next(
                (
                    item
                    for item in self.all_available_items
                    if item["name"] == item_text
                ),
                None,
            )
            if not existing:
                try:
                    # Create new item in DB
                    new_item = self.create_item_in_db(item_text)
                    if new_item:
                        self.all_available_items.append(new_item)
                        self.all_available_items.sort(key=lambda x: x["name"])
                except Exception as e:
                    messagebox.showerror(
                        "Error", f"Failed to create {self.item_type}: {str(e)}"
                    )
                    return

            self.current_items.append(item_text)
            self.update_display()
            self.entry.delete(0, tk.END)
            self.entry.focus()

            if self.callback:
                self.callback(self.current_items)

    def add_from_listbox(self, event=None):
        """Add selected item from listbox"""
        selection = self.listbox.curselection()
        if selection:
            item = self.listbox.get(selection[0])
            if item not in self.current_items:
                self.current_items.append(item)
                self.update_display()

                if self.callback:
                    self.callback(self.current_items)

    def remove_item(self, item_to_remove):
        """Remove an item from current selection"""
        if item_to_remove in self.current_items:
            self.current_items.remove(item_to_remove)
            self.update_display()

            if self.callback:
                self.callback(self.current_items)

    def update_display(self):
        """Update the display of current items"""
        # Clear existing
        for widget in self.display_frame.winfo_children():
            widget.destroy()

        if not self.current_items:
            tk.Label(
                self.display_frame, text=f"(no {self.item_type}s)", fg="gray"
            ).pack(anchor="w")
            return

        bg_color = "lightblue"

        for i, item in enumerate(self.current_items):
            item_frame = tk.Frame(self.display_frame)
            item_frame.pack(anchor="w", pady=1)

            tk.Label(item_frame, text=item, bg=bg_color, padx=5, pady=2).pack(
                side="left"
            )

            remove_btn = tk.Button(
                item_frame,
                text="×",
                font=("Arial", 8),
                command=lambda it=item: self.remove_item(it),
            )
            remove_btn.pack(side="left")

    def filter_list(self, event=None):
        """Filter the available items list"""
        filter_text = self.filter_entry.get().strip().lower()
        self.listbox.delete(0, tk.END)

        for item in self.all_available_items:
            if filter_text in item["name"].lower():
                self.listbox.insert(tk.END, item["name"])

    def load_all_items(self):
        """Load all available items from API"""
        try:
            url = TAGS_URL if self.item_type == "tag" else MATERIALS_URL
            response = requests.get(url, timeout=5)
            if response.status_code == 200:
                data = response.json()
                self.all_available_items = sorted(data, key=lambda x: x["name"])
                self.filter_list()  # Update listbox
            else:
                messagebox.showerror(
                    f"{self.item_type.title()}s Error",
                    f"Failed to load {self.item_type}s: {response.status_code}",
                )
        except Exception as e:
            messagebox.showerror(
                f"{self.item_type.title()}s Error",
                f"Error loading {self.item_type}s: {str(e)}",
            )

    def create_item_in_db(self, item_name):
        """Create new item in database via API"""
        try:
            url = TAGS_URL if self.item_type == "tag" else MATERIALS_URL
            payload = {"name": item_name}
            response = requests.post(url, json=payload, timeout=5)

            if response.status_code == 200:
                return response.json()
            else:
                messagebox.showerror(
                    "Error", f"Failed to create {self.item_type}: {response.text}"
                )
                return None
        except Exception as e:
            messagebox.showerror("Error", f"Error creating {self.item_type}: {str(e)}")
            return None

    def delete_unused_item(self):
        """Delete an unused item"""
        # Get selected item from listbox
        selection = self.listbox.curselection()
        if not selection:
            messagebox.showwarning(
                "Warning", f"Please select a {self.item_type} to delete"
            )
            return

        selected_item = self.listbox.get(selection[0])

        # Check if item is used
        if selected_item in self.current_items:
            messagebox.showwarning(
                "Warning", f"Cannot delete {self.item_type}: it is currently used"
            )
            return

        confirm = messagebox.askyesno(
            "Confirm Deletion",
            f"Are you sure you want to delete {self.item_type}: {selected_item}?",
        )
        if confirm:
            try:
                url = TAGS_URL if self.item_type == "tag" else MATERIALS_URL
                response = requests.delete(f"{url}/{selected_item}", timeout=5)

                if response.status_code == 200:
                    messagebox.showinfo(
                        "Success", f"{self.item_type.title()} deleted successfully"
                    )
                    # Remove from available items
                    self.all_available_items = [
                        item
                        for item in self.all_available_items
                        if item["name"] != selected_item
                    ]
                    self.filter_list()  # Update listbox
                else:
                    messagebox.showerror(
                        "Error", f"Failed to delete {self.item_type}: {response.text}"
                    )
            except Exception as e:
                messagebox.showerror(
                    "Error", f"Error deleting {self.item_type}: {str(e)}"
                )

    def get_current_items(self):
        """Get current selected items"""
        return self.current_items.copy()

    def set_items(self, items):
        """Set current items"""
        self.current_items = items.copy() if items else []
        self.update_display()

    def clear_items(self):
        """Clear all current items"""
        self.current_items = []
        self.update_display()

        if self.callback:
            self.callback(self.current_items)
