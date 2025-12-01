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
