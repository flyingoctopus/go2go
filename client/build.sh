#!/bin/bash

# Exit immediately if a command exits with a non-zero status.
set -e

echo "Building the project using wasm-pack..."

# Run wasm-pack build to compile the project to WebAssembly
wasm-pack build --target web

echo "Copying static files to the pkg directory..."

# Create the pkg directory if it doesn't exist
mkdir -p pkg

# Copy all contents from the static directory to the pkg directory
cp -r static/* pkg/

echo "Build and setup completed successfully!"
