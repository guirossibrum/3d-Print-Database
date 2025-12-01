import psycopg2

# Connect to the database
conn = psycopg2.connect("postgresql://admin:admin@localhost:5432/products")

# Create a cursor
cur = conn.cursor()

# Execute the ALTER TABLE command
cur.execute("ALTER TABLE products ADD COLUMN active BOOLEAN NOT NULL DEFAULT TRUE;")

# Commit the changes
conn.commit()

# Close the cursor and connection
cur.close()
conn.close()

print(
    "Successfully added 'active' column to products table with default TRUE for existing records."
)
