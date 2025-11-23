#!/bin/bash
# Fix permissions for Products directory to allow Docker container to write

echo "Setting up Products directory with proper permissions..."

# Create Products directory if it doesn't exist
mkdir -p /home/grbrum/Work/3d_print/Products

# Set proper permissions (775 for directories to allow group write, 664 for files)
chmod 775 /home/grbrum/Work/3d_print/Products
find /home/grbrum/Work/3d_print/Products -type d -exec chmod 775 {} \;
find /home/grbrum/Work/3d_print/Products -type f -exec chmod 664 {} \;

echo "Permissions set. Docker container should now be able to write to Products directory."
echo "Note: If you still have permission issues, you may need to run: sudo chown -R $USER:$USER /home/grbrum/Work/3d_print/Products"