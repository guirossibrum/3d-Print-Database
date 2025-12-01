# frontend/modules/toggles.py
"""Reusable toggle field components for boolean options"""

import tkinter as tk


class ToggleField:
    """Reusable toggle field component"""

    def __init__(
        self, parent, label_text, initial_value=False, row=0, column=0, **grid_options
    ):
        self.variable = tk.BooleanVar(value=initial_value)

        self.checkbox = tk.Checkbutton(
            parent, text=label_text, variable=self.variable, **grid_options
        )
        self.checkbox.grid(row=row, column=column, sticky="w", pady=5, padx=5)

    def get(self):
        """Get the boolean value"""
        return self.variable.get()

    def set(self, value):
        """Set the boolean value"""
        self.variable.set(value)

    def grid(self, **grid_options):
        """Configure grid layout"""
        self.checkbox.grid(**grid_options)

    def pack(self, **pack_options):
        """Configure pack layout"""
        self.checkbox.pack(**pack_options)


class ToggleGroup:
    """Group of related toggle fields"""

    def __init__(self, parent, fields_config, start_row=0, start_column=0):
        self.fields = {}
        self.parent = parent

        for i, config in enumerate(fields_config):
            field_name = config["name"]
            label_text = config["label"]
            initial_value = config.get("initial_value", False)

            # Create toggle field
            self.fields[field_name] = ToggleField(
                parent=parent,
                label_text=label_text,
                initial_value=initial_value,
                row=start_row + i,
                column=start_column,
                sticky="w",
                pady=5,
                padx=5,
            )

    def get_values(self):
        """Get all field values as dict"""
        return {name: field.get() for name, field in self.fields.items()}

    def set_values(self, values):
        """Set all field values from dict"""
        for name, value in values.items():
            if name in self.fields:
                self.fields[name].set(value)

    def get_field(self, name):
        """Get specific field by name"""
        return self.fields.get(name)

    def set_field(self, name, value):
        """Set specific field by name"""
        if name in self.fields:
            self.fields[name].set(value)


# Common toggle field configurations
PRODUCTION_ACTIVE_CONFIG = [
    {"name": "production", "label": "Production Ready", "initial_value": False},
    {"name": "active", "label": "Active", "initial_value": True},
]

SEARCH_FILTER_CONFIG = [
    {"name": "include_inactive", "label": "Include Inactive", "initial_value": False},
    {"name": "include_prototype", "label": "Include Prototype", "initial_value": False},
]


def create_production_active_group(parent, start_row=0, start_column=0):
    """Create production/active toggle group"""
    return ToggleGroup(
        parent=parent,
        fields_config=PRODUCTION_ACTIVE_CONFIG,
        start_row=start_row,
        start_column=start_column,
    )


def create_search_filter_group(parent, start_row=0, start_column=0):
    """Create search filter toggle group"""
    return ToggleGroup(
        parent=parent,
        fields_config=SEARCH_FILTER_CONFIG,
        start_row=start_row,
        start_column=start_column,
    )
