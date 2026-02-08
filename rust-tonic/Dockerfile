FROM rust:1.85-alpine as builder
WORKDIR /app
RUN apk add --no-cache musl-dev protoc
COPY . .
RUN cargo build --release

FROM alpine:3.21
WORKDIR /app
COPY --from=builder /app/target/release/server ./server
COPY --from=builder /app/target/release/client ./client
ENV RUST_LOG=info
ENV PORT=8080
EXPOSE 8080
CMD ["./server"]