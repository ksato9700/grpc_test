from concurrent import futures
import time
import grpc
from apis import helloworld_pb2
from apis import helloworld_pb2_grpc

_ONE_DAY_IN_SECONDS = 60 * 60 * 24


class Greeter(helloworld_pb2_grpc.GreeterServicer):

    def sayHello(self, request, context):
        print('request', request)
        print(repr(request.name))
        print(repr(request.ver))
        print(repr(request.bloodType))
        print('context', context)
        return helloworld_pb2.HelloReply(
            message='Hello {}!'.format(request.name))


def serve():
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
    helloworld_pb2_grpc.add_GreeterServicer_to_server(Greeter(), server)
    server.add_insecure_port('[::]:50051')
    server.start()

    try:
        while True:
            time.sleep(_ONE_DAY_IN_SECONDS)
    except KeyboardInterrupt:
        server.stop(0)


if __name__ == '__main__':
    serve()
