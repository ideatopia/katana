FROM alpine:latest AS base

ENV HOST=0.0.0.0
ENV PORT=8080
ENV ROOT_DIR=public
ENV WORKER=4

WORKDIR /app

COPY target/x86_64-unknown-linux-musl/release/katana ./katana

RUN chmod +x katana

EXPOSE $PORT

CMD ["sh", "-c","./katana", "--host", "${HOST}", "--port", "${PORT}", "--root-dir", "${ROOT_DIR}", "--worker", "${WORKER}"]
