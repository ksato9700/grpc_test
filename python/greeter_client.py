import os

import grpc
from greeter.helloworld import helloworld_pb2, helloworld_pb2_grpc


def run():
    server = os.environ.get("GRPC_SERVER", "localhost")
    port = os.environ.get("GRPC_PORT", "50051")
    insecure = os.environ.get("GRPC_INSECURE", False)
    cert = os.environ.get("GRPC_CERT", None)
    if insecure:
        channel = grpc.insecure_channel(f"{server}:{port}")
    else:
        with open(cert, "rb") as rfp:
            credentials = grpc.ssl_channel_credentials(rfp.read())
            channel = grpc.secure_channel(f"{server}:{port}", credentials)

    stub = helloworld_pb2_grpc.GreeterStub(channel)
    for name in ("Joe", "Bill", "Donald"):
        try:
            if name == "Bill":
                extra = helloworld_pb2.Extra(
                    extra_message="hey there",
                    extra_code=123,
                )
            else:
                extra = None
            response = stub.sayHello(
                helloworld_pb2.HelloRequest(
                    name=name, ver=123, bloodType="B", extra=extra
                )
            )
            # response = stub.sayHello(helloworld_pb2.HelloRequest(ver=123))
            print(f"greeter client received: {response.message}")
        except Exception as e:
            print(f"{e.code()}: {e.details()}")


if __name__ == "__main__":
    run()
