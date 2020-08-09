import grpc

from apis import helloworld_pb2
from apis import helloworld_pb2_grpc


def run():
    channel = grpc.insecure_channel('localhost:50051')
    stub = helloworld_pb2_grpc.GreeterStub(channel)
    response = stub.sayHello(helloworld_pb2.HelloRequest(
        name='you',
        ver=123,
        bloodType="AB"
    ))
    # response = stub.sayHello(helloworld_pb2.HelloRequest(ver=123))
    print('greeter client received: {}'.format(response.message))


if __name__ == '__main__':
    run()
