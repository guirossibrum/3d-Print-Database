# frontend/modules/tags.py
"""Tag management operations"""

import tkinter as tk
from tkinter import messagebox
import requests
from .constants import TAGS_URL


def add_tag(
    tag_entry, current_tags, update_display_func, all_available_tags, tag_listbox
):
    """Add a tag to current tags list"""
    tag_text = tag_entry.get().strip()
    if tag_text and tag_text not in current_tags:
        current_tags.append(tag_text)
        update_display_func()
        tag_entry.delete(0, tk.END)  # Clear the input
        tag_entry.focus()
        # Add to available tags immediately
        if tag_text not in all_available_tags:
            all_available_tags.append(tag_text)
            all_available_tags.sort()
            tag_listbox.delete(0, tk.END)
            for tag in all_available_tags:
                tag_listbox.insert(tk.END, tag)


def remove_tag(tag_to_remove, current_tags, update_display_func):
    """Remove a tag from current tags list"""
    if tag_to_remove in current_tags:
        current_tags.remove(tag_to_remove)
        update_display_func()


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
            command=lambda t=tag: remove_tag(
                t,
                tags_list,
                lambda: update_tag_display(tags_list, display_frame, layout),
            ),
        )
        remove_btn.pack(side=tk.LEFT)


def load_all_tags_for_list(all_available_tags_list):
    """Load all tags for the listbox"""
    try:
        response = requests.get(TAGS_URL, timeout=5)
        if response.status_code == 200:
            data = response.json()
            all_available_tags_list[:] = sorted(
                [tag["name"] for tag in data if "name" in tag]
            )
        else:
            messagebox.showerror(
                "Tags Error", f"Failed to load tags: {response.status_code}"
            )
    except Exception as e:
        messagebox.showerror("Tags Error", f"Error loading tags: {str(e)}")


def filter_tag_list(tag_filter_entry, all_available_tags, tag_listbox):
    """Filter the tag list based on entry"""
    filter_text = tag_filter_entry.get().strip().lower()
    tag_listbox.delete(0, tk.END)
    for tag in all_available_tags:
        if filter_text in tag.lower():
            tag_listbox.insert(tk.END, tag)


def add_tag_from_listbox(listbox, current_tags, update_func):
    """Generic helper to add a tag from the listbox"""
    selection = listbox.curselection()
    if selection:
        tag = listbox.get(selection[0])
        if tag not in current_tags:
            current_tags.append(tag)
            update_func(current_tags)


def delete_unused_tag(selected_tag, all_available_tags, tag_listbox, update_list_func):
    """Delete an unused tag"""
    if not selected_tag:
        messagebox.showwarning("Warning", "Please select a tag to delete")
        return

    # Check if tag is used
    try:
        # This would need API call to check usage, but for now assume
        confirm = messagebox.askyesno(
            "Confirm Deletion",
            f"Are you sure you want to delete tag: {selected_tag}?\n\n"
            "This will only delete if the tag is not used by any products.",
        )
        if confirm:
            # API call to delete
            response = requests.delete(f"{TAGS_URL}/{selected_tag}")
            if response.status_code == 200:
                messagebox.showinfo("Success", "Tag deleted successfully")
                if selected_tag in all_available_tags:
                    all_available_tags.remove(selected_tag)
                update_list_func()
            else:
                messagebox.showerror("Error", f"Failed to delete tag: {response.text}")
    except Exception as e:
        messagebox.showerror("Error", f"Error deleting tag: {str(e)}")


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
