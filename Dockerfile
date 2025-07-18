# Stage 1: Base builder
FROM rust:latest AS builder

WORKDIR /app

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools

COPY Cargo.toml Cargo.lock ./

COPY . .

RUN cargo build --release --target=x86_64-unknown-linux-musl

# Stage 2: Runtime
FROM alpine:latest

ENV KATANA_HOST=0.0.0.0
ENV KATANA_PORT=8080
ENV KATANA_DOCUMENT_ROOT=public
ENV KATANA_WORKER=4
ENV KATANA_LOG_LEVEL=info

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/katana ./

CMD ["./katana"]
