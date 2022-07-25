
protos:
	(cd ..; make);
	mkdir -p proto
	cp -p ../proto/*.proto proto

PROTOS=$(wildcard proto/*.proto)
JSPROTOS = $(PROTOS:.proto=.js)

proto/%.js: proto/%.proto
	cp -p $^ $@

build-docker: $(JSPROTOS)
	docker build . --target builder -t greeter-server-node-builder:latest
	docker build . -t greeter-server-node:latest

build-cloud:
	gcloud builds submit

run-server-local:
	yarn run run-server

run-server-docker:
	docker run -it --rm --name greeter-server-node -e PORT=8080 -p 50051:8080 greeter-server-node:latest

run-client:
	yarn run run-client
