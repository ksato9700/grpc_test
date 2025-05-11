#!/bin/bash
set -e

echo "Setting up uv for the greeter project..."

# Check if uv is installed
if ! command -v uv &> /dev/null; then
    echo "uv is not installed. Installing..."
    pip install uv
fi

# Create a virtual environment
echo "Creating a virtual environment..."
uv venv

# Activate the virtual environment
echo "Activating the virtual environment..."
if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
    source .venv/Scripts/activate
else
    source .venv/bin/activate
fi

# Install dependencies
echo "Installing dependencies..."
uv pip install -e .

# Install dev dependencies
echo "Installing dev dependencies..."
uv pip install -e ".[dev]"

echo "Setup complete! You can now run the server with 'make run-server' and the client with 'make run-client-local'."
