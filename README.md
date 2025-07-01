# Katana

A lightweight web server written in Rust, designed for serving static content with *`elegance`*.

> **Note:** This project started as a learning initiative but is being developed with "production-grade" standards in mind. While it's currently in development, we're working towards making it "production-ready". Contributions and feedback are welcome to help improve its reliability and feature set.

## Features

- Static file serving with directory listing
- Chunked file transfer for large files
- Range request support
- Configurable host, port, and root directory
- Multi-worker support
- Cross-platform compatibility (Windows, Linux, macOS)

## Quick Start

You can install Katana in several ways:

### 🔧 With Cargo (requires Rust)

```bash
cargo install --git https://github.com/ideatopia/katana
````

### 🐳 With Docker

```bash
docker build -t katana .
docker run -p 8080:8080 katana
```

## Configuration

Katana can be configured using command-line arguments:

### Common options:

| Flag             | Description                                 | Default       |
|------------------|---------------------------------------------|---------------|
| `--host`         | Bind address (e.g. `127.0.0.1`, `0.0.0.0`)  | `0.0.0.0`     |
| `--port`         | Port number to serve on                     | `8080`        |
| `--document-root`| Folder to serve                             | `./public`    |
| `--worker`       | Number of worker threads                    | `4`           |
| `--log-level`    | Log level: `DEBUG`, `INFO`, `WARN`, `ERROR` | `INFO`        |


Example:
```bash
katana --host 0.0.0.0 --port 8000 --document-root ./static --worker 8
```

## License

MIT License - See [LICENSE](LICENSE) for details.

## Author

Judicaël AHYI [ludndev](https://github.com/ludndev)
