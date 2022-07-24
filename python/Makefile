PROTOS_DIR=../proto
PROTOS=$(wildcard $(PROTOS_DIR)/*.proto)
OUTDIR=greeter/helloworld

GRPC_GEN=poetry run python -m grpc_tools.protoc
GRPC_OUTPUT_OPTS=--python_out=$(OUTDIR) --grpc_python_out=$(OUTDIR)

PB2S=$(patsubst $(PROTOS_DIR)/%.proto,$(OUTDIR)/%_pb2.py,$(PROTOS))

$(OUTDIR)/%_pb2.py:$(PROTOS_DIR)/%.proto
	$(GRPC_GEN) -I$(PROTOS_DIR) $(GRPC_OUTPUT_OPTS) $^

gen-grpc: $(PB2S)

build-docker: gen-grpc
	docker build . --target builder -t greeter-server-python-builder:latest
	docker build . -t greeter-server-python:latest

run-client-local:
	GRPC_INSECURE=True poetry run python greeter_client.py

# See https://pki.goog/repository/ to get certificate
# Specify the server name as GRPC_SERVER
run-client-cloud:
	GRPC_CERT=gtsr1.pem GRPC_PORT=443 poetry run python greeter_client.py

run-server:
	PORT=50051 poetry run python -m greeter.server

run-server-docker:
	docker run -it --rm -e PORT=8080 -p 50051:8080 greeter-server-python:latest
