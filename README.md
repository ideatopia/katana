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

### Running from Source

1. Clone the repository
2. Build and run:
```bash
cargo run
```

### Using Docker

```bash
docker build -t katana .
docker run -p 8080:8080 katana
```

## Configuration

Katana can be configured using command-line arguments:

- `--host`: Host address (default: 127.0.0.1 on Windows, 0.0.0.0 on others)
- `--port`: Port number (default: 8080)
- `--dir`: Root directory for serving files (default: "public")
- `--worker`: Number of workers (default: 4, minimum: 1)

Example:
```bash
katana --host 0.0.0.0 --port 3000 --dir ./static --worker 8
```

## License

MIT License - See [LICENSE](LICENSE) for details.

## Author

Judicaël AHYI [ludndev](https://github.com/ludndev)
