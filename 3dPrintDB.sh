#!/bin/bash
# Launch 3D Print Database GUI application
# This script detaches the GUI from the terminal so it runs independently

setsid python3 ~/Work/3d_print/Code/frontend/3dPrintDB.py >/dev/null 2>&1 &
disown
