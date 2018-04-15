GRPC_GEN=python -m grpc_tools.protoc
OUTPUT_OPTS=--python_out=. --grpc_python_out=.

PROTOS_DIR=../protos
PROTOS=$(wildcard $(PROTOS_DIR)/*.proto)
PB2S=$(patsubst $(PROTOS_DIR)/%.proto,%_pb2.py,$(PROTOS))

%_pb2.py:$(PROTOS_DIR)/%.proto
	$(GRPC_GEN) -I$(PROTOS_DIR) $(OUTPUT_OPTS) $(PROTOS_DIR)/helloworld.proto

all: $(PB2S)
