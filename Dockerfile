FROM rust:1.78.0 as builder

WORKDIR /app
COPY . .

RUN cargo build --release

FROM debian:buster-slim

COPY --from=builder /app/target/release/grpc-server /usr/local/bin/grpc-server

EXPOSE 50051

CMD ["grpc-server"]