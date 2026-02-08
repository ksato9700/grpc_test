# gRPC Agents

This repository contains multiple implementations (agents) of the Greeter service using gRPC. Each agent can act as both a server and a client, allowing for cross-language communication.

## Agent Overview

| Language | Directory | Build System | Server Command | Client Command |
| :--- | :--- | :--- | :--- | :--- |
| **Java** | `java/` | Gradle | `make run-server` | `make run-client` |
| **Node.js** | `node/` | NPM / Yarn | `make run-server-local` | `make run-client` |
| **Python** | `python/` | uv / pip | `make run-server` | `make run-client-local` |
| **Rust** | `rust-tonic/` | Cargo | `make run-server-local` | `make run-client` |

---

## Shared Protocol Buffers

All agents use the protocol buffer definitions located in the `proto/` directory:
- `helloworld.proto`: Service and message definitions.
- `blood_type.proto`: Shared enum definitions.

---

## Java Agent

### Prerequisites
- JDK 17 or higher
- Gradle (optional, `gradlew` is provided)

### Setup & Run
```bash
cd java
make build        # Setup protos and build
make run-server   # Start the server
make run-client   # Run the client (use ARGS="..." for arguments)
```

---

## Node.js Agent

### Prerequisites
- Node.js (v22+ recommended)
- pnpm

### Setup & Run
```bash
cd node
pnpm install
make run-server-local
make run-client       # use ARGS="..." for arguments
```

---

## Python Agent

### Prerequisites
- Python 3.10+
- `uv` (recommended) or `pip`

### Setup & Run
```bash
cd python
./setup_uv.sh     # Setup virtualenv and install dependencies
make gen-grpc     # Generate gRPC code
make run-server
make run-client-local
```

---

## Rust Agent

### Prerequisites
- Rust (latest stable)
- Cargo

### Setup & Run
```bash
cd rust-tonic
make protos       # Copy protos to local directory
make build        # Build the project
make run-server-local
make run-client
```

---

## Root Makefile Shortcuts

For convenience, you can run commands from the root directory:

```bash
make build-java        # Build Java agent
make build-python      # Build Python agent
make build-node        # Build Node agent
make build-rust        # Build Rust agent

make run-server-java   # Run Java server
make run-server-node   # Run Node server
make run-server-rust   # Run Rust server

make all               # Build all agents
```