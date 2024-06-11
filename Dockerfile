FROM rust:1.78.0-alpine3.20 as builder

RUN apk add --no-cache curl=8.7.1-r0 unzip=6.0-r14 build-base=0.5-r3

ARG TARGETPLATFORM

RUN case "$TARGETPLATFORM" in \
    "linux/amd64") \
        curl -LO https://github.com/protocolbuffers/protobuf/releases/download/v27.1/protoc-27.1-linux-x86_64.zip && \
        unzip protoc-27.1-linux-x86_64.zip -d /usr/local && \
        rm protoc-27.1-linux-x86_64.zip \
        ;; \
    "linux/arm64") \
        curl -LO https://github.com/protocolbuffers/protobuf/releases/download/v27.1/protoc-27.1-linux-aarch_64.zip && \
        unzip protoc-27.1-linux-aarch_64.zip -d /usr/local && \
        rm protoc-27.1-linux-aarch_64.zip \
        ;; \
    *) echo "Unsupported platform: $TARGETPLATFORM" && exit 1 ;; \
    esac

ENV PATH=$PATH:/usr/local/bin

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY . .

RUN cargo build --release

FROM nginx:1.21.3-alpine as runtime

RUN apk add --no-cache supervisor wget

ARG TARGETPLATFORM
RUN case "$TARGETPLATFORM" in \
    "linux/amd64") \
        wget -qO /usr/bin/grpc-health-probe https://github.com/grpc-ecosystem/grpc-health-probe/releases/download/v0.4.26/grpc_health_probe-linux-amd64 \
        ;; \
    "linux/arm64") \
        wget -qO /usr/bin/grpc-health-probe https://github.com/grpc-ecosystem/grpc-health-probe/releases/download/v0.4.26/grpc_health_probe-linux-arm64 \
        ;; \
    *) echo "Unsupported platform: $TARGETPLATFORM" && exit 1 ;; \
    esac \
    && chmod +x /usr/bin/grpc-health-probe \
    && apk del wget

COPY ./configure/supervisord.conf /etc/supervisor/conf.d/supervisord.conf

COPY ./configure/default.conf /etc/nginx/conf.d/default.conf

COPY --from=builder /app/target/release/grpc-server /usr/local/bin/grpc-server

EXPOSE 80 8080 50051

CMD ["supervisord", "-c", "/etc/supervisor/conf.d/supervisord.conf"]
