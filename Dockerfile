FROM rust:1.78.0-bullseye as builder

RUN apt-get update && apt-get install -y protobuf-compiler

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY . .

RUN cargo build --release

FROM debian:bullseye

RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/grpc-server /usr/local/bin/grpc-server
## descriptor file
COPY --from=builder /app/proto/hello_descriptor.bin /usr/local/bin/proto/hello_descriptor.bin

EXPOSE 50051

CMD ["grpc-server"]