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

ENV HOST=0.0.0.0
ENV PORT=8080
ENV ROOT_DIR=public
ENV WORKER=4

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/katana ./

CMD ["sh", "-c","./katana", "--host", "${HOST}", "--port", "${PORT}", "--root-dir", "${ROOT_DIR}", "--worker", "${WORKER}"]
