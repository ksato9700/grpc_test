# Agents

This repository contains multiple implementations of the Greeter service (Agent) using gRPC. Below are the instructions to run the client and server for each language.

## Java Agent

Located in `java/`.

### Build
```bash
cd java
make build
# or directly: ./gradlew installDist
```

### Run Server
```bash
make run-server
# or directly: app/build/install/app/bin/hello-world-server
```

### Run Client
```bash
make run-client
# or directly: app/build/install/app/bin/hello-world-client
```

## Node Agent

Located in `node/`.

### Install Dependencies
```bash
cd node
npm install
# or
yarn install
```

### Run Server
```bash
npm run run-server
# or
yarn run-server
```

### Run Client
```bash
npm run run-client
# or
yarn run-client
```

## Python Agent

Located in `python/`.

### Setup
```bash
cd python
./setup_uv.sh
```

### Run Server
```bash
make run-server
```

### Run Client
```bash
make run-client-local
```

## Rust Agent

Located in `rust-tonic/`.

### Run Server
```bash
cd rust-tonic
make run-server-local
# or
cargo run --bin server
```

### Run Client
```bash
make run-client
# or
cargo run --bin client
```
