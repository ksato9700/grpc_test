# Project Overview: grpc_test

## Purpose

`grpc_test` is a polyglot reference repository demonstrating the same gRPC service implemented across multiple languages. Each language directory provides a self-contained server and client that speak the same Protocol Buffer contract, making it useful for testing cross-language gRPC interoperability and comparing idiomatic gRPC usage patterns across ecosystems.

The service is a simple `Greeter` that responds to `sayHello` requests. All implementations share an intentional test case: requests with the name `"Donald"` are rejected with `INVALID_ARGUMENT`, exercising error-handling paths consistently across every language.

## Directory Structure

```
grpc_test/
├── proto/                  # Shared Protocol Buffer definitions
│   ├── helloworld.proto    # Greeter service + message definitions
│   └── blood_type.proto    # BloodType enum used in HelloRequest
├── java/                   # Java implementation (Gradle + grpc-java)
├── node/                   # TypeScript/Node.js implementation (@grpc/grpc-js)
├── python/                 # Python implementation (grpcio, uv-managed)
├── ruby/                   # Ruby implementation (grpc gem)
├── rust-tonic/             # Rust implementation (tonic, async/tokio)
│   └── proto/              # Local copy of protos for tonic-build
├── Makefile                # Root shortcuts delegating to sub-makefiles
├── AGENTS.md               # Build & run instructions for all agents
├── protodep.toml           # Proto dependency manager config
└── README.md
```

## Tech Stack

| Category        | Technology                                     |
|-----------------|------------------------------------------------|
| Protocol        | gRPC (unary RPC only)                          |
| Schema          | Protocol Buffers v3                            |
| Java            | JDK 21, grpc-java 1.61.1, Gradle               |
| Node.js         | TypeScript 5.7, @grpc/grpc-js 1.13, pnpm, Vite |
| Python          | Python 3.13, grpcio 1.78, uv                   |
| Ruby            | Ruby 3.2+, grpc gem 1.78                       |
| Rust            | Rust stable, tonic 0.12, tokio, Cargo          |
| Build (root)    | GNU Make + protodep                            |
| Containerization| Docker (per-language Dockerfiles)              |
| CI              | Google Cloud Build (cloudbuild.yaml, some langs)|

## Key Files

| File | Role |
|------|------|
| `proto/helloworld.proto` | Service contract: `Greeter.sayHello`, `HelloRequest`, `HelloReply` |
| `proto/blood_type.proto` | Shared `BloodType` enum (A/B/O/AB) |
| `rust-tonic/src/lib.rs` | Rust service implementation + unit/integration tests |
| `python/greeter/server.py` | Python service implementation |
| `python/test_greeter.py` | Python integration test (server+client in-process) |
| `java/app/src/main/java/helloworld/HelloWorldServer.java` | Java service implementation |
| `node/src/server.ts` | TypeScript service implementation |
| `ruby/server.rb` | Ruby service implementation |
| `AGENTS.md` | Primary usage guide for all language agents |
| `Makefile` | Root-level build/run shortcuts |

## Dependencies

| Language | Key Packages |
|----------|-------------|
| Java | `io.grpc:grpc-protobuf:1.61.1`, `io.grpc:grpc-netty-shaded`, `com.google.protobuf:protobuf-java-util:3.25.3` |
| Node.js | `@grpc/grpc-js ^1.13.3`, `@grpc/proto-loader ^0.7.15`, dev: `typescript`, `vite`, `biome` |
| Python | `grpcio >=1.78.0`, `protobuf >=6.33.0`, `click >=8.2.0`, dev: `ruff`, `grpcio-tools`, `pytest` |
| Ruby | `grpc ~>1.78.0`, `google-protobuf ~>4.33.5`, `standard` |
| Rust | `tonic 0.12`, `prost 0.13`, `tokio 1.42`, `clap`, `tracing`, build: `tonic-build` |

## Setup & Usage

Each language is independently runnable. Refer to `AGENTS.md` for full instructions. Quick reference:

```bash
# Python
cd python && ./setup_uv.sh && make gen-grpc && make run-server

# Rust
cd rust-tonic && make protos && make build && make run-server-local

# Node.js
cd node && pnpm install && make run-server-local

# Java
cd java && make build && make run-server

# Ruby
cd ruby && make build && make run-server
```

Root-level shortcuts are available via `make run-server-<lang>` and `make run-client-<lang>`.

Cross-language testing: start a server in one language and run the client from any other — all share the same proto contract and default port `50051`.
