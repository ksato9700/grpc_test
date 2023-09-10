FROM rust:alpine3.18 as builder
WORKDIR /work
RUN apk add --no-cache cmake make build-base protoc
COPY build.rs Cargo.toml Cargo.lock /work/
COPY proto /work/proto
COPY src /work/src
RUN cargo build --release --bin server

FROM alpine:3.18
COPY --from=builder /work/target/release/server /server
CMD ["./server"]
