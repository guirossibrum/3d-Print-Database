# frontend/modules/inventory.py
"""Inventory management functionality"""

import tkinter as tk
from tkinter import messagebox
import requests
from .constants import INVENTORY_URL, API_URL


def load_inventory_status(inventory_text_widget, tree=None):
    """Load inventory status for all products"""
    try:
        response = requests.get(INVENTORY_URL, timeout=5)
        if response.status_code == 200:
            products = response.json()
            display_inventory_status(products, inventory_text_widget, tree)
        else:
            inventory_text_widget.delete(1.0, tk.END)
            inventory_text_widget.insert(
                tk.END, f"Error: {response.status_code} - {response.text}"
            )
    except Exception as e:
        inventory_text_widget.delete(1.0, tk.END)
        inventory_text_widget.insert(tk.END, f"Error: {str(e)}")


def display_inventory_status(products, inventory_text_widget, tree=None):
    """Display inventory status in text widget or tree"""
    if tree:
        # Use tree widget for better display
        display_inventory_tree(products, tree)
    else:
        # Fallback to text widget
        display_inventory_text(products, inventory_text_widget)


def display_inventory_text(products, inventory_text_widget):
    """Display inventory status in text widget"""
    inventory_text_widget.delete(1.0, tk.END)

    if not products:
        inventory_text_widget.insert(tk.END, "No products found.")
        return

    # Calculate totals
    total_value = 0
    total_products = len(products)

    for product in products:
        stock_qty = product.get("stock_quantity", 0)
        unit_cost = product.get("unit_cost", 0)
        if stock_qty and unit_cost:
            total_value += stock_qty * unit_cost

    # Display summary
    inventory_text_widget.insert(tk.END, f"Total Products: {total_products}\n")
    inventory_text_widget.insert(
        tk.END, f"Total Inventory Value: ${total_value / 100:.2f}\n\n"
    )

    # Display individual products
    for product in products:
        sku = product.get("sku", "N/A")
        name = product.get("name", "N/A")
        stock_qty = product.get("stock_quantity", 0)
        reorder_point = product.get("reorder_point", 0)
        unit_cost = product.get("unit_cost", 0)
        selling_price = product.get("selling_price", 0)

        # Calculate status
        if stock_qty == 0:
            status = "OUT OF STOCK"
        elif stock_qty <= reorder_point:
            status = "LOW STOCK"
        else:
            status = "IN STOCK"

        # Calculate profit margin
        profit_margin = None
        if unit_cost and selling_price and unit_cost > 0:
            profit_margin = ((selling_price - unit_cost) / unit_cost) * 100

        inventory_text_widget.insert(tk.END, f"{sku} - {name}\n")
        inventory_text_widget.insert(
            tk.END, f"  Stock: {stock_qty} | Status: {status}\n"
        )
        inventory_text_widget.insert(
            tk.END,
            f"  Cost: ${unit_cost / 100:.2f} | Price: ${selling_price / 100:.2f}",
        )
        if profit_margin is not None:
            inventory_text_widget.insert(tk.END, f" | Margin: {profit_margin:.1f}%\n")
        inventory_text_widget.insert(tk.END, "\n")


def display_inventory_tree(products, tree):
    """Display inventory in tree widget with sorting"""
    # Clear existing items
    for item in tree.get_children():
        tree.delete(item)

    # Add products to tree
    for product in products:
        sku = product.get("sku", "N/A")
        name = product.get("name", "N/A")
        stock_qty = product.get("stock_quantity", 0)
        reorder_point = product.get("reorder_point", 0)
        unit_cost = product.get("unit_cost", 0)
        selling_price = product.get("selling_price", 0)

        # Calculate status
        if stock_qty == 0:
            status = "Out of Stock"
        elif stock_qty <= reorder_point:
            status = "Low Stock"
        else:
            status = "In Stock"

        # Calculate values
        total_value = stock_qty * unit_cost if stock_qty and unit_cost else 0
        profit_margin = None
        if unit_cost and selling_price and unit_cost > 0:
            profit_margin = ((selling_price - unit_cost) / unit_cost) * 100
        else:
            profit_margin = 0

        tree.insert(
            "",
            "end",
            values=(
                sku,
                name,
                stock_qty,
                f"${unit_cost / 100:.2f}" if unit_cost else "$0.00",
                f"${selling_price / 100:.2f}" if selling_price else "$0.00",
                f"${total_value / 100:.2f}",
                f"{profit_margin:.1f}%" if profit_margin is not None else "0.0%",
                status,
            ),
        )


def sort_inventory_column(tree, column, reverse=False):
    """Sort inventory tree by column"""
    try:
        # Get all items
        items = [(tree.set(item, column), item) for item in tree.get_children("")]

        # Sort by column value
        if column in ["Cost", "Price", "Total Value", "Margin"]:
            # Sort numeric columns
            def get_numeric(value):
                try:
                    # Remove $ and % and convert to float
                    clean_value = value.replace("$", "").replace("%", "")
                    return float(clean_value)
                except:
                    return 0.0

            items.sort(key=lambda x: get_numeric(x[0]), reverse=reverse)
        else:
            # Sort text columns
            items.sort(key=lambda x: x[0].lower(), reverse=reverse)

        # Reorder items in tree
        for index, (value, item) in enumerate(items):
            tree.move(item, "", index)
    except Exception as e:
        messagebox.showerror("Sort Error", f"Error sorting inventory: {str(e)}")


def adjust_inventory_dialog(parent, product, callback=None):
    """Show inventory adjustment dialog"""
    dialog = tk.Toplevel(parent)
    dialog.title("Adjust Inventory")
    dialog.geometry("400x300")
    dialog.transient(parent)
    dialog.grab_set()

    # Product info
    tk.Label(dialog, text=f"Product: {product.get('name', 'N/A')}").pack(pady=10)
    tk.Label(dialog, text=f"Current Stock: {product.get('stock_quantity', 0)}").pack()

    # Adjustment frame
    adj_frame = tk.Frame(dialog)
    adj_frame.pack(pady=20)

    tk.Label(adj_frame, text="Adjustment:").grid(row=0, column=0, padx=5)

    adjustment_var = tk.StringVar(value="1")
    adjustment_spin = tk.Spinbox(
        adj_frame, from_=-100, to=100, textvariable=adjustment_var, width=10
    )
    adjustment_spin.grid(row=0, column=1, padx=5)

    tk.Label(adj_frame, text="Operation:").grid(row=1, column=0, padx=5, pady=10)

    operation_var = tk.StringVar(value="printed")
    operation_frame = tk.Frame(adj_frame)
    operation_frame.grid(row=1, column=1, padx=5, pady=10)

    tk.Radiobutton(
        operation_frame, text="Printed", variable=operation_var, value="printed"
    ).pack(side="left")
    tk.Radiobutton(
        operation_frame, text="Sold", variable=operation_var, value="sold"
    ).pack(side="left")

    def apply_adjustment():
        try:
            quantity = int(adjustment_var.get())
            operation = operation_var.get()
            current_stock = product.get("stock_quantity", 0)

            if operation == "sold" and quantity > current_stock:
                messagebox.showerror("Error", "Cannot sell more than current stock")
                return

            # Calculate new stock
            if operation == "printed":
                new_stock = current_stock + quantity
            else:  # sold
                new_stock = current_stock - quantity

            # Update via API
            payload = {"stock_quantity": new_stock}
            response = requests.put(
                f"{API_URL}{product['id']}", json=payload, timeout=5
            )

            if response.status_code == 200:
                messagebox.showinfo(
                    "Success", f"Inventory adjusted: {operation} {quantity}"
                )
                if callback:
                    callback()
                dialog.destroy()
            else:
                messagebox.showerror(
                    "Error", f"Failed to update inventory: {response.text}"
                )
        except ValueError:
            messagebox.showerror("Error", "Please enter a valid quantity")
        except Exception as e:
            messagebox.showerror("Error", f"Error adjusting inventory: {str(e)}")

    # Buttons
    button_frame = tk.Frame(dialog)
    button_frame.pack(pady=20)

    tk.Button(button_frame, text="Apply", command=apply_adjustment).pack(
        side="left", padx=5
    )
    tk.Button(button_frame, text="Cancel", command=dialog.destroy).pack(
        side="left", padx=5
    )

    dialog.wait_window()
