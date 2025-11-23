FROM python:3.11-slim

# Set working directory
WORKDIR /app

# Copy and install dependencies
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

# Copy backend code
COPY . .

# Create Products directory
RUN mkdir -p /Products

# Set umask for proper file permissions
RUN echo 'umask 002' >> /root/.bashrc

# Expose FastAPI port
EXPOSE 8000

# Start Uvicorn server with proper umask
CMD ["sh", "-c", "umask 002 && uvicorn app.main:app --host 0.0.0.0 --port 8000"]
