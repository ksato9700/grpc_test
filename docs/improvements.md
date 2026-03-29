# Improvement Proposals: grpc_test

> Analyzed: 2026-03-29
> Repository: /Users/ksato/git/grpc_test

## Summary

All identified issues have been resolved. See the Priority Matrix for details.

---

## 1. Code Quality & Refactoring

**~~Java: missing `!` in reply message~~** (`java/app/src/main/java/helloworld/HelloWorldServer.java:91`) — **Fixed.**

`"Hello " + req.getName()` corrected to `"Hello " + req.getName() + "!"`, consistent with all other implementations.

**~~Node.js: type safety gaps~~** (`node/src/server.ts`, `node/src/client.ts`) — **Fixed.**

Added `HelloRequest` and `HelloReply` interfaces to `proto.ts`. `server.ts` now uses `grpc.handleUnaryCall<HelloRequest, HelloReply>` instead of `UntypedHandleCall`. `client.ts` callback response typed as `HelloReply` instead of `any`.

**~~Rust Cargo.toml stale versions~~** (`rust-tonic/Cargo.toml`) — **Fixed.**

Bumped to `tonic = "0.13"` (resolves 0.13.1), `prost = "0.14"`, `tokio = "1.45"` (resolves 1.50.0), `futures = "0.3.31"`, `tonic-build = "0.13"`. Cargo.lock updated.

---

## 2. Security

**~~No security disclaimer~~** — **Fixed.**

Added a security note to `README.md` and a "Note" callout in the Cross-Language Interoperability section of `AGENTS.md`, documenting that all implementations intentionally use insecure (plaintext) connections and should not be deployed in shared environments.

---

## 3. Performance

**~~Python `serve_forever` busy-waits via `time.sleep`~~** (`python/greeter/server.py`) — **Fixed.**

Replaced the `while True: time.sleep(86400)` loop with `server.wait_for_termination()`. Removed the unused `_ONE_DAY_IN_SECONDS` constant and `import time`.

**~~Node.js client fires callbacks without awaiting~~** (`node/src/client.ts`) — **Fixed.**

Each `sayHello` call is now wrapped in a `Promise` and `await`ed sequentially. `main()` is `async`. Responses can no longer be silently lost on process exit.

---

## 4. Documentation Gaps

**~~No cross-language interop instructions~~** — **Fixed.**

Added a "Cross-Language Interoperability" section to `AGENTS.md` with concrete examples, including how to start a server in one language and connect with a client from another.

**~~Stale Rust setup instructions~~** (`AGENTS.md`) — **Fixed.**

Removed the `make protos` step from the Rust setup instructions; `build.rs` now reads `../proto/` automatically for local builds.

---

## 5. Architecture

**~~Duplicated proto files~~** (`rust-tonic/proto/` vs `proto/`) — **Fixed.**

`build.rs` auto-detects the proto source: uses `../proto/` for local builds, falls back to `proto/` inside the Docker build context.

**~~No automated tests for Node.js and Ruby~~** — **Fixed.**

- Node.js: `src/server.test.ts` using vitest (2 tests: happy path + Donald rejection).
- Ruby: `test_greeter.rb` using minitest (2 tests, 4 assertions).

**~~Java dependencies behind Python/Ruby~~** — **Fixed.**

Bumped to `grpcVersion = '1.80.0'` and `protobufVersion = '4.34.1'`, aligned with Python (grpcio 1.78) and Ruby (grpc 1.78).

---

## Priority Matrix

| Priority  | Item                                              | Effort |
|-----------|---------------------------------------------------|--------|
| ✅ Done   | Java reply missing `!` (breaks cross-lang tests)  | S      |
| ✅ Done   | Rust proto duplication (silent stale builds)      | S      |
| ✅ Done   | Java dependency versions behind Python/Ruby       | S      |
| ✅ Done   | Node.js missing tests                             | M      |
| ✅ Done   | Ruby missing tests                                | M      |
| ✅ Done   | Node.js type safety (UntypedHandleCall / any)     | M      |
| ✅ Done   | Python `serve_forever` — use `wait_for_termination` | S    |
| ✅ Done   | Node.js async client callback coordination        | S      |
| ✅ Done   | Cross-language interop docs in AGENTS.md          | S      |
| ✅ Done   | TLS note in README / security disclaimer          | S      |
| ✅ Done   | Verify/update Rust Cargo.toml dependency versions | S      |
