import sys
from pathlib import Path

sys.path.append(str(Path(__file__).parent))

__all__ = [
    'helloworld_pb2',
    'helloworld_pb2_grpc',
]

from . import helloworld_pb2
from . import helloworld_pb2_grpc
