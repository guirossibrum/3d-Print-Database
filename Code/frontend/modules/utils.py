# frontend/modules/utils.py
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

    # If it's the placeholder, leave it
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


def complete_time_with_underscores(entry, text):
    """Complete time with underscores for formatting"""
    # Implementation...


def complete_partial_time(entry, text):
    """Complete partial time input"""
    # Implementation...


def format_time_complete(entry):
    """Format time entry completely"""
    # Implementation...


def on_time_key_release_popup(event):
    """Handle key release for time input in popup"""
    # Implementation...


def format_time_input(entry, placeholder):
    """Format time input"""
    # Implementation...


def clear_form():
    """Clear the form fields"""
    # Implementation...


def add_tag():
    """Add a tag to the current tags"""
    # Implementation...


def remove_tag(tag_to_remove):
    """Remove a tag from current tags"""
    # Implementation...


def update_tag_display():
    """Update the tag display"""
    # Implementation...


def load_all_tags_for_list():
    """Load all tags for the listbox"""
    # Implementation...


def filter_tag_list(event=None):
    """Filter the tag list"""
    # Implementation...


def add_tag_from_list(event=None):
    """Add tag from list"""
    # Implementation...


def delete_unused_tag():
    """Delete unused tag"""
    # Implementation...


def load_categories():
    """Load categories from API"""
    # Implementation...


def update_category_dropdown():
    """Update category dropdown"""
    # Implementation...


def on_category_select(event):
    """Handle category selection"""
    # Implementation...


def load_inventory_status():
    """Load inventory status"""
    # Implementation...


def search_products():
    """Search products"""
    # Implementation...


def display_search_results():
    """Display search results"""
    # Implementation...


def load_product_from_search():
    """Load product from search for editing"""
    # Implementation...
