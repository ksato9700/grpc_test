import logging
import os
import time
from concurrent import futures

import grpc

from .helloworld import helloworld_pb2 as pb2
from .helloworld import helloworld_pb2_grpc as pb2_grpc

_ONE_DAY_IN_SECONDS = 60 * 60 * 24

logger = logging.getLogger(__name__)
logging.basicConfig(level=logging.INFO)


class Greeter(pb2_grpc.GreeterServicer):
    def sayHello(self, request, context):
        logging.info(f"{request=}")
        logging.info(f"{request.name=}")
        logging.info(f"{request.ver=}")
        logging.info(f"{request.bloodType=}")

        return pb2.HelloReply(message=f"Hello {request.name}!")


def serve():
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
    port = os.environ.get("PORT", "50051")
    pb2_grpc.add_GreeterServicer_to_server(Greeter(), server)
    server_url = f"[::]:{port}"
    server.add_insecure_port(server_url)
    logger.info(f"gRPC server running at {server_url}")
    server.start()

    try:
        while True:
            time.sleep(_ONE_DAY_IN_SECONDS)
    except KeyboardInterrupt:
        server.stop(0)


if __name__ == "__main__":
    serve()
