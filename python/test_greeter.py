import time
from multiprocessing import Process

from greeter.server import serve
from greeter_client import main


def run_client():
    main.callback(
        server="localhost",
        port=50051,
        insecure=True,
        cert_path=None,
        name=("Joe", "Bill", "Donald"),
    )


def test_greeter():
    server = serve()
    try:
        p = Process(target=run_client)
        p.start()
        p.join()
        assert p.exitcode == 0
    finally:
        server.stop(0)
        time.sleep(1)