import os

import click
import grpc
from greeter.helloworld import helloworld_pb2, helloworld_pb2_grpc


def get_channel(server: str, port: int, insecure: bool, cert_path: str | None):
    if insecure:
        return grpc.insecure_channel(f"{server}:{port}")
    else:
        if cert_path is None:
            raise click.UsageError("cert_path must be provided when insecure is False")
        with open(cert_path, "rb") as rfp:
            credentials = grpc.ssl_channel_credentials(rfp.read())
            return grpc.secure_channel(f"{server}:{port}", credentials)


@click.command()
@click.option("--server", default=os.environ.get("GRPC_SERVER", "localhost"))
@click.option("--port", default=os.environ.get("GRPC_PORT", 50051), type=int)
@click.option("--insecure", default=os.environ.get("GRPC_INSECURE", False), is_flag=True)
@click.option("--cert-path", default=os.environ.get("GRPC_CERT", None), type=click.Path())
@click.argument("name", type=str, nargs=-1)
def main(server: str, port: int, insecure: bool, cert_path: str | None, name: tuple[str]):
    """Greeter client."""
    channel = get_channel(server, port, insecure, cert_path)
    stub = helloworld_pb2_grpc.GreeterStub(channel)
    for n in name:
        try:
            if n == "Bill":
                extra = helloworld_pb2.Extra(
                    extra_message="hey there",
                    extra_code=123,
                )
            else:
                extra = None
            response = stub.sayHello(
                helloworld_pb2.HelloRequest(name=n, ver=123, bloodType="B", extra=extra)
            )
            print(f"greeter client received: {response.message}")
        except Exception as e:
            print(f"{e.code()}: {e.details()}")


if __name__ == "__main__":
    main()
