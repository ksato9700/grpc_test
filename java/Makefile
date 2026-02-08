PROTO_DIR = app/src/main/proto

setup:
	mkdir -p $(PROTO_DIR)
	if [ ! -L $(PROTO_DIR) ]; then \
		rm -rf $(PROTO_DIR); \
		ln -s ../../../../proto $(PROTO_DIR); \
	fi

build: setup
	./gradlew installDist

build-docker: build
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
