# gRPC Greeter Test

This repository contains a simple gRPC Greeter service implementation in multiple languages:

- **Java**: `java/`
- **Node.js**: `node/`
- **Python**: `python/`
- **Rust**: `rust-tonic/`
- **Ruby**: `ruby/`

## Getting Started

See [AGENTS.md](./AGENTS.md) for detailed instructions on how to build and run the client and server for each language.

## Protocol Buffers

The Protocol Buffer definitions are located in the `proto/` directory.

- `helloworld.proto`: Defines the Greeter service and messages.
- `blood_type.proto`: Defines an enum used in the messages.

## Security Note

All implementations use **insecure (plaintext) connections** — no TLS or authentication. This is intentional for a local development and testing harness. Do not expose these servers in shared or production environments.
