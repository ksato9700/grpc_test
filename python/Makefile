PROTOS_DIR=../proto
PROTOS=$(wildcard $(PROTOS_DIR)/*.proto)
OUTDIR=helloworld

GRPC_GEN=poetry run python -m grpc_tools.protoc
GRPC_OUTPUT_OPTS=--python_out=$(OUTDIR) --grpc_python_out=$(OUTDIR)

PB2S=$(patsubst $(PROTOS_DIR)/%.proto,$(OUTDIR)/%_pb2.py,$(PROTOS))

$(OUTDIR)/%_pb2.py:$(PROTOS_DIR)/%.proto
	$(GRPC_GEN) -I$(PROTOS_DIR) $(GRPC_OUTPUT_OPTS) $^

gen-grpc: $(PB2S)

run-client:
	poetry run python greeter_client.py

run-server:
	poetry run python greeter_server.py
