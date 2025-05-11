FROM python:3.11 as builder
WORKDIR /work
RUN pip install --upgrade pip uv
COPY greeter greeter
COPY pyproject.toml pyproject.toml
RUN uv venv; uv pip install .
# COPY helloworld /usr/local/lib/python3.11/site-packages/helloworld

FROM python:3.11-slim
COPY --from=builder /work/.venv/lib/python3.11/site-packages/ /usr/local/lib/python3.11/site-packages
# ENV PYTHONPATH=/usr/local/lib/python3.11/site-packages/helloworld
CMD ["python", "-m", "greeter.server"]
