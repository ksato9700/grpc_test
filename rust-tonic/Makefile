protos:
	cp -rp ../proto .

build-docker: protos
	docker build . --target builder -t greeter-server-rust-builder:latest
	docker build . -t greeter-server-rust:latest

run-server-local:
	cargo run --bin server

run-server-docker:
	docker run -it --rm -e PORT=8080 -p 50051:8080 greeter-server-rust:latest

run-client:
	cargo run --bin client
