PROTOS_DIR=../proto
PROTOS=$(wildcard $(PROTOS_DIR)/*.proto)
OUTDIR=greeter/helloworld

PYTHON=uv run python

GRPC_GEN=uv run python -m grpc_tools.protoc
GRPC_OUTPUT_OPTS=--python_out=$(OUTDIR) --grpc_python_out=$(OUTDIR)

PB2S=$(patsubst $(PROTOS_DIR)/%.proto,$(OUTDIR)/%_pb2.py,$(PROTOS))

$(OUTDIR)/%_pb2.py:$(PROTOS_DIR)/%.proto
	$(GRPC_GEN) -I$(PROTOS_DIR) $(GRPC_OUTPUT_OPTS) $^

gen-grpc: $(PB2S)
	gsed -i '/^import .*pb.*/s/import /from . import /' greeter/helloworld/*.py

build-docker: gen-grpc
	docker-buildx build . --target builder -t greeter-server-python-builder:latest
	docker-buildx build . -t greeter-server-python:latest

run-client-local:
	GRPC_INSECURE=True $(PYTHON) greeter_client.py Joe Bill Donald

# See https://pki.goog/repository/ to get certificate
# Specify the server name as GRPC_SERVER
run-client-cloud:
	GRPC_CERT=gtsr1.pem GRPC_PORT=443 $(PYTHON) greeter_client.py Joe Bill Donald

# Usage: make run-client ARGS="--insecure Donald"
run-client:
	$(PYTHON) greeter_client.py $(ARGS)

run-server:
	PORT=50051 $(PYTHON) -m greeter.server

run-server-docker:
	docker run -it --rm -e PORT=8080 -p 50051:8080 greeter-server-python:latest
