# Improvement Proposals: grpc_test

> Analyzed: 2026-03-29
> Repository: /Users/ksato/git/grpc_test

## Summary

- ~~Java server returns `"Hello NAME"` (missing `!`) while all other implementations return `"Hello NAME!"`.~~ **Fixed.**
- ~~`rust-tonic/proto/` duplicates `proto/` at the repo root — any proto change must be updated in two places.~~ **Fixed.**
- Java gRPC/protobuf dependencies are significantly behind the versions used by Python and Ruby.
- Node.js and Ruby have no automated tests; only Rust has thorough unit + integration coverage.
- Node.js loses type safety in both server (`UntypedHandleCall`) and client (`response: any`).

---

## 1. Code Quality & Refactoring

### Issues

**~~Java: missing `!` in reply message~~** (`java/app/src/main/java/helloworld/HelloWorldServer.java:91`) — **Fixed.**

`"Hello " + req.getName()` corrected to `"Hello " + req.getName() + "!"`, consistent with all other implementations.

**Node.js: type safety gaps** (`node/src/server.ts:4`, `node/src/client.ts:23`)

```ts
// server.ts — UntypedHandleCall disables request/response type checking
const sayHello: grpc.UntypedHandleCall = (call, callback) => { ... }

// client.ts — `any` loses the response type
client.sayHello({ name }, (error: grpc.ServiceError | null, response: any) => { ... });
```

**Rust Cargo.toml shows stale versions** (`rust-tonic/Cargo.toml`)

The Cargo.toml still lists `tonic = "0.12"` and `tokio = "1.42"` despite a commit (`6b8f99b`) claiming they were bumped to 0.13.1 and 1.45.1. The file appears to not have been saved during that update. Run `cargo update` and commit the refreshed `Cargo.lock`.

### Recommendations

- For Node.js, switch from dynamic proto loading to static code generation (e.g., `ts-proto` or `@grpc/proto-loader` with typed stubs) so `UntypedHandleCall` and `response: any` can be replaced with generated types.
- Verify and update `rust-tonic/Cargo.toml` to match the stated dependency versions.

---

## 2. Security

### Issues

All five implementations bind with **insecure credentials** (no TLS, no authentication):

- Python: `server.add_insecure_port(...)` (`python/greeter/server.py:37`)
- Node.js: `grpc.ServerCredentials.createInsecure()` (`node/src/server.ts:27`)
- Ruby: `:this_port_is_insecure` (`ruby/server.rb:26`)
- Rust: no `tls_config()` call (`rust-tonic/src/server.rs:34`)
- Java: `ServerBuilder.forPort(port)` with no credentials (`java/.../HelloWorldServer.java:35`)

### Recommendations

This is acceptable for a local test harness, but worth documenting explicitly in `README.md` or `AGENTS.md` so consumers don't accidentally deploy these servers in a shared environment. If TLS examples are desired, the Rust/tonic implementation is the natural starting point (`ServerTlsConfig` + `ClientTlsConfig`).

---

## 3. Performance

### Issues

**Python `serve_forever` busy-waits via `time.sleep`** (`python/greeter/server.py:47-50`)

```python
while True:
    time.sleep(_ONE_DAY_IN_SECONDS)  # 86400 seconds
```

While not actively harmful, this pattern is unusual — the main thread just sleeps indefinitely. A `threading.Event` or `server.wait_for_termination()` is the idiomatic grpcio approach.

**Node.js client fires callbacks asynchronously without awaiting** (`node/src/client.ts:22-29`)

All `sayHello` calls are dispatched in a loop without coordination. If the process exits before all callbacks complete, responses are silently lost. This is visible when greeting multiple names.

### Recommendations

- Python: replace the `while/sleep` loop with `server.wait_for_termination()`.
- Node.js: use `Promise`-wrapped calls or the `@grpc/grpc-js` promisify utility and `await` each call (or `Promise.all` for parallel dispatch).

---

## 4. Documentation Gaps

### Issues

- **No cross-language test instructions**: `AGENTS.md` explains how to run each server/client individually but doesn't give a concrete example of running, say, the Python server with the Rust client.
- **Root `README.md` is minimal**: it lists the languages but doesn't mention the shared proto contract, the `BloodType` field, the `Extra` optional message, or the `"Donald"` rejection behavior.
- **No test documentation for Java**: `java/app/src/test/java/helloworld/HelloWorldClientTest.java` exists but isn't referenced in `AGENTS.md`.

### Recommendations

- Add a "Cross-language interop" section to `AGENTS.md` with a worked example (e.g., start Rust server, run Python client).
- Document the `"Donald"` rejection behavior and the `Extra` optional field in `README.md` or `proto/helloworld.proto` comments — these are non-obvious test behaviors.

---

## 5. Architecture

### Issues

**~~Duplicated proto files~~** (`rust-tonic/proto/` vs `proto/`) — **Fixed.**

`build.rs` now auto-detects the proto source: uses `../proto/` when running locally (eliminating the need for `make protos` during development), and falls back to `rust-tonic/proto/` inside the Docker build context where `make build-docker` still copies them via its `protos` dependency.

**No automated tests for Node.js and Ruby**

- Node.js (`node/src/`): no test files found.
- Ruby (`ruby/`): no test files found.
- By contrast, Rust has 3 tests (2 unit + 1 integration) and Python has an integration test.

**Java dependencies are behind**

`java/app/build.gradle:18-19` pins `grpcVersion = '1.61.1'` and `protobufVersion = '3.25.3'`, while Python and Ruby both ship with grpc 1.78.x and protobuf 4.x/6.x. This divergence could surface proto-encoding edge cases in cross-language tests.

### Recommendations

- **Tests**: add at minimum a simple integration test for Node.js (e.g., using `vitest` to spin up the server and assert a successful response) and a Ruby RSpec or minitest covering the happy path and the `"Donald"` rejection.
- **Java dependencies**: bump `grpcVersion` to `1.68+` and `protobufVersion` to `4.x` to align with the other implementations.

---

## Priority Matrix

| Priority  | Item                                              | Effort |
|-----------|---------------------------------------------------|--------|
| ✅ Done   | Java reply missing `!` (breaks cross-lang tests)  | S      |
| ✅ Done   | Rust proto duplication (silent stale builds)      | S      |
| 🟡 Medium | Java dependency versions behind Python/Ruby       | S      |
| 🟡 Medium | Node.js missing tests                             | M      |
| 🟡 Medium | Ruby missing tests                                | M      |
| 🟡 Medium | Node.js type safety (UntypedHandleCall / any)     | M      |
| 🟡 Medium | Python `serve_forever` — use `wait_for_termination` | S    |
| 🟡 Medium | Node.js async client callback coordination        | S      |
| 🟢 Low    | Cross-language interop docs in AGENTS.md          | S      |
| 🟢 Low    | TLS note in README / security disclaimer          | S      |
| 🟢 Low    | Verify/update Rust Cargo.toml dependency versions | S      |
