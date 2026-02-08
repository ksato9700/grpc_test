protos:
	rm -rf proto
	mkdir -p proto
	cp -rp ../proto/*.proto proto

build-docker: protos
	docker build . -t greeter-server-rust:latest

run-server-local:
	RUST_LOG=info cargo run --bin server

run-server-docker:
	docker run -it --rm \
		--name greeter-server-rust \
		-e PORT=8080 \
		-e RUST_LOG=info \
		-p 50051:8080 \
		greeter-server-rust:latest

build:
	cargo build

run-client:
	RUST_LOG=info cargo run --bin client -- Ken Bill Donald

test:
	cargo test

test-docker: protos
	docker build . --target builder -t greeter-server-rust-builder
	docker run --rm greeter-server-rust-builder cargo test
