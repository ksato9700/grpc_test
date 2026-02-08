PROTO_DIR = app/src/main/proto

setup:
	rm -rf $(PROTO_DIR)
	mkdir -p $(PROTO_DIR)
	cp ../proto/*.proto $(PROTO_DIR)/

build: setup build-local

build-local:
	docker run --rm -v $(CURDIR):/app -w /app amazoncorretto:21 ./gradlew installDist

build-docker: setup
	docker build . -t greeter-server-java:latest

run-client:
	app/build/install/app/bin/hello-world-client $(ARGS)

run-server:
	app/build/install/app/bin/hello-world-server

run-server-docker:
	docker run -it --rm -e PORT=8080 -p 50051:8080 greeter-server-java:latest

test:
	./gradlew test

lint:
	./gradlew spotlessCheck

format:
	./gradlew spotlessApply
