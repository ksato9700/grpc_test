proto_files = \
	proto/blood_type.proto \
	proto/helloworld.proto

$(proto_files): protodep.toml
	protodep up --use-https -f -c -i unused

.PHONY: help
help:
	@echo "Usage: make [target]"
	@echo ""
	@echo "Targets:"
	@echo "  proto-up       Update proto files using protodep"
	@echo "  build-java     Build Java implementation"
	@echo "  build-python   Build Python implementation"
	@echo "  build-node     Build Node.js implementation"
	@echo "  build-rust     Build Rust implementation"
	@echo "  run-server-java   Run Java server"
	@echo "  run-client-java   Run Java client (use ARGS=\"...\" for args)"
	@echo "  test-java      Run Java tests"
	@echo "  run-server-node   Run Node server"
	@echo "  run-client-node   Run Node client (use ARGS=\"...\" for args)"
	@echo "  run-server-rust   Run Rust server"
	@echo "  run-client-rust   Run Rust client (use ARGS=\"...\" for args)"
	@echo "  all            Build all implementations"

.PHONY: proto-up
proto-up: $(proto_files)

.PHONY: build-java
build-java:
	$(MAKE) -C java build

.PHONY: build-python
build-python:
	$(MAKE) -C python build

.PHONY: build-node
build-node:
	$(MAKE) -C node build

.PHONY: build-rust
build-rust:
	$(MAKE) -C rust-tonic build

.PHONY: run-server-java
run-server-java:
	$(MAKE) -C java run-server

.PHONY: run-client-java
run-client-java:
	$(MAKE) -C java run-client ARGS="$(ARGS)"

.PHONY: test-java
test-java:
	$(MAKE) -C java test

.PHONY: run-server-node
run-server-node:
	$(MAKE) -C node run-server-local

.PHONY: run-client-node
run-client-node:
	$(MAKE) -C node run-client ARGS="$(ARGS)"

.PHONY: run-server-rust
run-server-rust:
	$(MAKE) -C rust-tonic run-server-local

.PHONY: run-client-rust
run-client-rust:
	$(MAKE) -C rust-tonic run-client ARGS="$(ARGS)"

.PHONY: all
all: build-java build-python build-node build-rust
