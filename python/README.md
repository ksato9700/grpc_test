# Greeter gRPC Service

This is a simple gRPC service that demonstrates how to use gRPC with Python.

## Setup with uv

This project uses [uv](https://github.com/astral-sh/uv) for dependency management.

### Installation

You can use the provided setup script to install uv and set up the project:

```bash
./setup_uv.sh
```

Or manually:

1. Install uv:

```bash
pip install uv
```

2. Create a virtual environment and install dependencies:

```bash
# Create and activate a virtual environment
uv venv
source .venv/bin/activate  # On Windows: .venv\Scripts\activate

# Install dependencies
uv pip install -e .

# Install dev dependencies
uv pip install -e ".[dev]"
```

Alternatively, you can use the requirements.txt file:

```bash
uv pip install -r requirements.txt
# For dev dependencies
uv pip install -r requirements.txt[dev]
```

### Usage

Run the server:

```bash
make run-server
```

Run the client:

```bash
make run-client-local
```

### Docker

Build and run with Docker:

```bash
make build-docker
make run-server-docker
```

### Development

Run linting:

```bash
make lint
```

Run formatting:

```bash
make format
```

Run tests:

```bash
make test
```

## Project Structure

- `greeter/` - The main package
  - `server.py` - The gRPC server implementation
  - `helloworld/` - Generated gRPC code
- `greeter_client.py` - The gRPC client implementation
- `pyproject.toml` - Project configuration
- `requirements.txt` - Dependencies for uv
