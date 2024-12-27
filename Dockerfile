# Multi-stage build for Frontend
FROM node:20 AS frontend-builder
WORKDIR /app
# Dynamically initialize the Node.js project
RUN npm init -y
# Install Monaco Editor
RUN npm install monaco-editor
# Copy the frontend code
COPY frontend/ ./
# Create dist directory and copy frontend files
RUN mkdir -p /app/dist && \
    cp -r index.html canvas.html /app/dist/ && \
    cp -r ./node_modules/monaco-editor/min /app/dist/

# Final stage: Build and run backend
FROM rust:1.72-slim
WORKDIR /app
# Create workspace directory
RUN mkdir -p /workspace
# Copy backend code
COPY backend/ ./backend/
WORKDIR /app/backend
# Build the backend
RUN cargo build --release
# Copy the frontend files
COPY --from=frontend-builder /app/dist /app/frontend
# Install Python for serving frontend
RUN apt-get update && apt-get install -y --no-install-recommends python3 && apt-get clean

# Expose both ports
EXPOSE 8080 3000
# Serve the frontend via Python's built-in HTTP server and run the backend
CMD ["sh", "-c", "./target/release/backend & cd /app/frontend && python3 -m http.server 3000"]
