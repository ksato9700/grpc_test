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
        if request.name == "Donald":
            context.set_details(f"Ouch! I don't like you, {request.name}")
            context.set_code(grpc.StatusCode.INVALID_ARGUMENT)
            return pb2.HelloReply()

        logging.info(f"{request=}")
        logging.info(f"{request.name=}")
        logging.info(f"{request.ver=}")
        logging.info(f"{request.bloodType=}")
        logging.info(f"{request.extra}")

        return pb2.HelloReply(message=f"Hello {request.name}!")


def serve(port: str = "50051"):
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
    pb2_grpc.add_GreeterServicer_to_server(Greeter(), server)
    server_url = f"[::]:{port}"
    server.add_insecure_port(server_url)
    logger.info(f"gRPC server running at {server_url}")
    server.start()
    return server


def serve_forever():
    port = os.environ.get("PORT", "50051")
    server = serve(port)
    try:
        while True:
            time.sleep(_ONE_DAY_IN_SECONDS)
    except KeyboardInterrupt:
        server.stop(0)


if __name__ == "__main__":
    serve_forever()
