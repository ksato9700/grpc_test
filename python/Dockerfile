FROM python:3.10 as builder
WORKDIR /work
RUN pip install --upgrade pip
COPY greeter greeter
COPY pyproject.toml pyproject.toml
RUN pip install .
# COPY helloworld /usr/local/lib/python3.10/site-packages/helloworld

FROM python:3.10-slim
COPY --from=builder /usr/local/lib/python3.10/site-packages /usr/local/lib/python3.10/site-packages
# ENV PYTHONPATH=/usr/local/lib/python3.10/site-packages/helloworld
CMD ["python", "-m", "greeter.server"]
