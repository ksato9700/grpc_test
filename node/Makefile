build: protos
	pnpm install

protos:
	mkdir -p proto
	cp -p ../proto/*.proto proto

build-docker: protos
	docker build . -t greeter-server-node:latest

run-server-local: protos
	pnpm dev:server

run-server-docker:
	docker run -it --rm --name greeter-server-node -e PORT=8080 -p 50051:8080 greeter-server-node:latest

run-client:
	pnpm dev:client Joe Bill Donald

lint:
	pnpm lint

format:
	pnpm format
