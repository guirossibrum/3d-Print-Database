# frontend/modules/ui_components.py
"""Reusable UI components"""

import tkinter as tk
from tkinter import messagebox


class CheckRating(tk.Frame):
    """5-point rating using [ x ] style — perfectly aligned, no layout jump"""

    def __init__(self, parent, initial_rating=0, callback=None):
        super().__init__(parent)
        self.rating = initial_rating
        self.callback = callback
        self.buttons = []

        # Use a monospaced font so [   ] and [ x ] have identical width
        self.font = ("DejaVu Sans Mono", 18, "bold")  # or "Consolas", "Courier New"
        # Alternative great fonts: "Fira Code", "Source Code Pro", "Ubuntu Mono"

        for i in range(1, 6):  # ratings 1 to 5
            btn = tk.Label(
                self,
                text="     ",
                font=self.font,
                fg="black",
                bg="#f0f0f0",
                relief="solid",
                borderwidth=1,
                width=5,  # Fixed width in characters
                cursor="hand2",
            )
            btn.pack(side=tk.LEFT, padx=2)

            # Hover effect
            btn.bind("<Enter>", lambda e, b=btn: b.config(bg="#ffffe0"))
            btn.bind("<Leave>", lambda e, b=btn: b.config(bg="#f0f0f0"))

            # Click handling
            btn.bind("<Button-1>", lambda e, n=i: self.set_rating(n))

            self.buttons.append(btn)

        self.update_display()

    def set_rating(self, rating):
        # Toggle behavior: click same rating → decrease by 1 (like stars)
        if self.rating == rating:
            self.rating = max(0, rating - 1)
        else:
            self.rating = rating

        self.update_display()
        if self.callback:
            self.callback(self.rating)

    def update_display(self):
        for i, btn in enumerate(self.buttons):
            if i < self.rating:
                btn.config(text="  X  ", fg="black")
            else:
                btn.config(text="     ", fg="black")

    def get_rating(self):
        return self.rating

    def set_rating_direct(self, rating):
        self.rating = max(0, min(5, int(rating)))
        self.update_display()


class TimeEntry(tk.Frame):
    """Enhanced time entry widget with smart formatting"""

    def __init__(self, parent, placeholder="__:__"):
        super().__init__(parent)
        self.placeholder = placeholder

        self.entry = tk.Entry(self, width=10)
        self.entry.pack(side="left")

        # Set placeholder
        self.entry.insert(0, placeholder)
        self.entry.config(fg="gray")

        # Bind events
        self.entry.bind("<FocusIn>", self.on_focus_in)
        self.entry.bind("<FocusOut>", self.on_focus_out)
        self.entry.bind("<KeyRelease>", self.on_key_release)

    def on_focus_in(self, event):
        """Handle focus in"""
        current_text = self.entry.get()
        if current_text == self.placeholder:
            self.entry.delete(0, tk.END)
            self.entry.config(fg="black")

    def on_focus_out(self, event):
        """Handle focus out with formatting"""
        from .utils import format_time_complete

        format_time_complete(self.entry)

    def on_key_release(self, event):
        """Handle key release for live formatting"""
        from .utils import format_time_input_live

        # Schedule formatting to avoid interfering with typing
        self.after(100, lambda: format_time_input_live(self.entry))

    def get(self):
        """Get entry value"""
        return self.entry.get()

    def delete(self, start, end=None):
        """Delete entry content"""
        self.entry.delete(start, end)

    def insert(self, index, text):
        """Insert text into entry"""
        self.entry.insert(index, text)

    def config(self, **kwargs):
        """Configure entry widget"""
        self.entry.config(**kwargs)

    def focus(self):
        """Set focus to entry"""
        self.entry.focus()


class ErrorDialog:
    """Reusable error dialog with copyable text"""

    def __init__(self, parent, title, message):
        self.dialog = tk.Toplevel(parent)
        self.dialog.title(title)
        self.dialog.geometry("500x300")

        # Error icon and title
        header_frame = tk.Frame(self.dialog)
        header_frame.pack(pady=10, padx=10, fill="x")

        # Simple error icon using text
        tk.Label(header_frame, text="⚠", font=("Arial", 24), fg="red").pack(
            side=tk.LEFT, padx=5
        )
        tk.Label(header_frame, text=title, font=("Arial", 14, "bold")).pack(
            side=tk.LEFT, padx=10
        )

        # Text widget for copyable message
        text_frame = tk.Frame(self.dialog)
        text_frame.pack(fill="both", expand=True, padx=10, pady=(0, 10))

        self.text_widget = tk.Text(text_frame, wrap=tk.WORD, height=10, padx=5, pady=5)
        scrollbar = tk.Scrollbar(text_frame, command=self.text_widget.yview)
        self.text_widget.config(yscrollcommand=scrollbar.set)

        self.text_widget.pack(side=tk.LEFT, fill="both", expand=True)
        scrollbar.pack(side=tk.RIGHT, fill=tk.Y)

        self.text_widget.insert(tk.END, message)
        self.text_widget.config(state=tk.DISABLED)  # Make read-only but selectable

        # Button frame
        button_frame = tk.Frame(self.dialog)
        button_frame.pack(pady=10)

        def copy_to_clipboard():
            """Copy the error message to clipboard"""
            parent.clipboard_clear()
            parent.clipboard_append(message)
            # Optional: show brief feedback
            self.copy_btn.config(text="Copied!")
            self.dialog.after(1000, lambda: self.copy_btn.config(text="Copy"))

        self.copy_btn = tk.Button(button_frame, text="Copy", command=copy_to_clipboard)
        self.copy_btn.pack(side=tk.LEFT, padx=5)

        tk.Button(button_frame, text="OK", command=self.dialog.destroy).pack(
            side=tk.LEFT, padx=5
        )

        # Make dialog modal
        self.dialog.transient(parent)
        self.dialog.grab_set()

    def show(self):
        """Show the dialog"""
        self.dialog.wait_window()
