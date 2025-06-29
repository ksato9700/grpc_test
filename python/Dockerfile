FROM python:3.13 as builder
WORKDIR /work
RUN pip install --upgrade pip uv
COPY greeter greeter
COPY pyproject.toml pyproject.toml
RUN uv venv; uv pip install .

FROM python:3.13-slim
COPY --from=builder /work/.venv/lib/python3.13/site-packages/ /usr/local/lib/python3.13/site-packages
CMD ["python", "-m", "greeter.server"]
