# frontend/modules/utils.py
"""General utility functions"""

import tkinter as tk


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

    # If it's placeholder, leave it
    if current_text == "__:__":
        return

    # If empty, show placeholder
    if not current_text:
        entry.delete(0, tk.END)
        entry.insert(0, "__:__")
        entry.config(fg="gray")
        return

    # Check if it's already in HH:MM format
    if ":" in current_text and len(current_text.split(":")) == 2:
        parts = current_text.split(":")
        hours_part = parts[0]
        minutes_part = parts[1]

        # Validate and format
        try:
            hours = int(hours_part) if hours_part else 0
            minutes = int(minutes_part) if minutes_part else 0

            minutes = min(minutes, 59)

            formatted = f"{hours:02d}:{minutes:02d}"
            entry.delete(0, tk.END)
            entry.insert(0, formatted)
            entry.config(fg="black")
            return
        except ValueError:
            pass  # Fall through to completion formatting

    # Complete any partial formatting
    format_time_complete(entry)


def format_time_complete(entry):
    """Format time entry completely"""
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

    # If it's placeholder, don't format
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


def show_copyable_error(title, message, root):
    """Show error dialog with copyable text using Text widget"""
    dialog = tk.Toplevel()
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


def add_copy_menu_to_entry(entry_widget, root):
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

    def copy_entry_text(entry_widget):
        """Copy text from an Entry widget to clipboard"""
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

    entry_widget.bind("<Button-3>", show_menu)  # Right-click
    entry_widget.bind("<Control-c>", lambda e: copy_entry_text(entry_widget))  # Ctrl+C
    entry_widget.bind("<Control-v>", lambda e: paste_to_entry(entry_widget))  # Ctrl+V

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
    production = production_toggles["production"].get()
    active = production_toggles["active"].get()

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