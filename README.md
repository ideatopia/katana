# Katana

<div align="center">

```
__  __     ______     ______   ______     __   __     ______
/\ \/ /    /\  __ \   /\__  _\ /\  __ \   /\ "-.\ \   /\  __ \
\ \  _"-.  \ \  __ \  \/_/\ \/ \ \  __ \  \ \ \-.  \  \ \  __ \
 \ \_\ \_\  \ \_\ \_\    \ \_\  \ \_\ \_\  \ \_\\"\_\  \ \_\ \_\
  \/_/\/_/   \/_/\/_/     \/_/   \/_/\/_/   \/_/ \/_/   \/_/\/_/
```

**A lightweight web server written in Rust, designed for serving static content with _elegance_**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)](https://www.rust-lang.org)
[![GitHub issues](https://img.shields.io/github/issues/ideatopia/katana)](https://github.com/ideatopia/katana/issues)

[Features](#features) • [Installation](#installation) • [Usage](#usage) • [Configuration](#configuration) • [Contributing](#contributing) • [License](#license)

</div>

---

## About

Katana is a high-performance, minimalist web server built from scratch in Rust. While it started as a learning project, it's being developed with production-grade standards in mind. Perfect for serving static websites, file sharing, or as a development server.

> **Note:** This project is currently in active development. While functional, it's not yet recommended for production use. Contributions and feedback are highly welcome!

## Features

### Core Capabilities
- **Static File Serving** - Fast and efficient file delivery
- **Directory Listing** - Automatic, themed directory browsing with dark mode support
- **Chunked Transfer** - Optimized handling of large files (1KB chunks)
- **Range Requests** - Support for partial content delivery (HTTP 206)
- **Flexible Configuration** - Multiple configuration sources (CLI, env vars, config file)
- **Multi-threading** - Configurable worker threads for concurrent connections
- **Cross-platform** - Works on Windows, Linux, and macOS

### HTTP Features
- **HTTP Methods**: GET, HEAD, OPTIONS, TRACE
- **HTTP Versions**: HTTP/1.0, HTTP/1.1 support
- **Content Types**: Comprehensive MIME type detection
- **Error Handling**: Beautiful, themed error pages with dark mode

### Developer Experience
- **Colorful Logging** - Terminal output with ANSI color support
- **Security** - Protection against directory traversal, hidden files filtering
- **Request Logging** - Detailed request/response logging
- **Docker Support** - Ready-to-use containerization

## Installation

### Option 1: Install via Cargo (Recommended)

```bash
cargo install --git https://github.com/ideatopia/katana
```

### Option 2: Docker

```bash
# Build the image
docker build -t katana .

# Run the container
docker run -p 8080:8080 -v $(pwd)/public:/app/public katana
```

### Option 3: Build from Source

```bash
# Clone the repository
git clone https://github.com/ideatopia/katana.git
cd katana

# Build in release mode
cargo build --release

# Binary will be in target/release/katana
./target/release/katana
```

## Usage

### Quick Start

```bash
# Start with defaults (serves ./public on http://0.0.0.0:8080)
katana

# Specify custom options
katana --host 127.0.0.1 --port 3000 --document-root ./static --worker 8
```

### Command-Line Options

**`--host <ADDRESS>`**

Specifies the network address the server will bind to. Use `127.0.0.1` for local access only (recommended for development), or `0.0.0.0` to accept connections from any network interface.

- Default: `0.0.0.0` on Unix-like systems (Linux, macOS)
- Default: `127.0.0.1` on Windows
- Examples:
  ```bash
  katana --host 127.0.0.1          # Local access only
  katana --host 0.0.0.0            # Accept from any interface
  katana --host 192.168.1.100      # Bind to specific IP
  ```

**`--port <NUMBER>`**

Sets the TCP port number the server will listen on. Choose ports above 1024 to avoid requiring administrator privileges.

- Default: `8080`
- Range: `1-65535`
- Examples:
  ```bash
  katana --port 3000               # Common alternative port
  katana --port 8000               # Development server port
  katana --port 80                 # Standard HTTP (requires admin rights)
  ```

**`--document-root <PATH>`**

Defines the root directory from which files will be served. Can be an absolute or relative path. The server will only serve files within this directory and its subdirectories.

- Default: `./public`
- Examples:
  ```bash
  katana --document-root ./static          # Relative path
  katana --document-root /var/www/html     # Absolute path
  katana --document-root ~/Documents/site  # Home directory path
  ```

**`--worker <NUMBER>`**

Controls the number of worker threads used to handle concurrent connections. More workers can improve performance under high load but will consume more system resources.

- Default: `4`
- Recommended: Number of CPU cores or 2x CPU cores for I/O-bound workloads
- Examples:
  ```bash
  katana --worker 2                # Low resource usage
  katana --worker 8                # High concurrency
  katana --worker 16               # Very high traffic
  ```

**`--log-level <LEVEL>`**

Sets the minimum severity level for log messages. Higher levels produce less output.

- Default: `INFO`
- Available levels: `DEBUG`, `INFO`, `WARN`, `ERROR`
- Level descriptions:
    - `DEBUG`: Detailed diagnostic information for development
    - `INFO`: General informational messages about server operations
    - `WARN`: Warning messages for potentially problematic situations
    - `ERROR`: Error messages for serious problems
- Examples:
  ```bash
  katana --log-level DEBUG         # Maximum verbosity
  katana --log-level INFO          # Standard output
  katana --log-level WARN          # Warnings and errors only
  katana --log-level ERROR         # Errors only
  ```

### Combining Options

You can combine multiple options to customize the server behavior:

```bash
# Development setup with detailed logging
katana --host 127.0.0.1 --port 3000 --document-root ./dist --log-level DEBUG

# Production-like setup with high concurrency
katana --host 0.0.0.0 --port 80 --document-root /var/www/html --worker 16 --log-level WARN

# File sharing server
katana --host 0.0.0.0 --port 8080 --document-root ~/Downloads --worker 4 --log-level INFO
```

## Configuration

Katana supports multiple configuration sources with the following priority (highest to lowest):

1. **Command-line arguments** (highest priority)
2. **Environment variables**
3. **Configuration file** (`.katana`)
4. **Default values** (lowest priority)

### Configuration File

Create a `.katana` file in your project root:

```toml
[katana]
# Server host (use "127.0.0.1" for local access only)
host = "127.0.0.1"

# Port number to run the server on
port = 8080

# Directory to serve static files from
document_root = "public"

# Number of worker threads for handling requests
worker = 4

# Logging level: DEBUG, INFO, WARN, ERROR
log_level = "INFO"
```

### Environment Variables

```bash
export KATANA_HOST=0.0.0.0
export KATANA_PORT=8080
export KATANA_DOCUMENT_ROOT=public
export KATANA_WORKER=4
export KATANA_LOG_LEVEL=INFO
```

### Docker Configuration

When using Docker, you can configure via environment variables:

```bash
docker run -p 8080:8080 \
  -e KATANA_HOST=0.0.0.0 \
  -e KATANA_PORT=8080 \
  -e KATANA_DOCUMENT_ROOT=public \
  -e KATANA_WORKER=4 \
  -e KATANA_LOG_LEVEL=info \
  -v $(pwd)/public:/app/public \
  katana
```

## Examples

### Serving a Static Website

```bash
# Navigate to your website directory
cd  /var/www/html/

# Start the server
katana --port 8000 --document-root .
```

or directly specify the path:

```bash
katana --port 8000 --document-root  /var/www/html/
```

### File Sharing Server 

```bash
# Share files from a specific directory
katana --document-root ~/Downloads --port 8080 --host 0.0.0.0
```

### High-Traffic Configuration

Based on your CPU cores, you can adjust the worker count for better performance:

```bash
# Optimize for concurrent connections
katana --worker 16 --port 80 --document-root /var/www/html
```

## Architecture

### Project Structure

```
katana/
├── src/
│   ├── core/
│   │   ├── config/         # Configuration management
│   │   ├── server/         # HTTP server implementation
│   │   ├── resources/      # Templates and static resources
│   │   └── utils/          # Utility functions (logger, colorful output, etc.)
│   ├── lib.rs              # Library entry point
│   └── main.rs             # Binary entry point
├── templates/              # HTML templates (error, directory listing, banner)
├── tests/                  # Unit and integration tests
├── .katana.example         # Example configuration file
├── Dockerfile              # Docker configuration
└── Cargo.toml              # Rust dependencies
```

### Key Components

- **Config System**: Multi-source configuration with priority handling
- **HTTP Parser**: Custom HTTP/1.x request parser
- **File Handler**: Efficient file serving with chunked transfer and range support
- **Template Engine**: Simple placeholder-based template system
- **Logger**: Colorful, level-based logging with timestamp formatting

## Security Considerations

- **Directory Traversal Protection**: Prevents access to files outside the document root
- **Hidden Files Filtering**: Automatically blocks access to files starting with `.` (except `.well-known`)
- **Safe Defaults**: Localhost binding on Windows by default
- **No Code Execution**: Serves only static files, no server-side scripting

> **Important**: This server is designed for serving static content only. Do not use it to serve sensitive data without additional security measures (HTTPS, authentication, etc.)

## Contributing

We welcome contributions! Here's how you can help:

### Ways to Contribute

1. **Report Bugs**: Open an issue with detailed reproduction steps
2. **Suggest Features**: Share your ideas for improvements
3. **Improve Documentation**: Help make the docs clearer
4. **Submit Pull Requests**: Fix bugs or add features

### Development Setup

```bash
# Clone the repository
git clone https://github.com/ideatopia/katana.git
cd katana

# Create a new branch
git checkout -b feature/your-feature-name

# Make your changes and test
cargo build
cargo test

# Commit with clear messages
git commit -m "feat(): add your feature description"

# Push and create a pull request
git push origin feature/your-feature-name
```

### Code Style

- Follow Rust's official style guidelines
- Run `cargo fmt` before committing
- Ensure `cargo clippy` passes without warnings
- Add tests for new features
- Update documentation as needed

## Roadmap

### Completed
- [x] Multi-threading support
- [x] Range request support (HTTP 206)
- [x] Chunked transfer encoding
- [x] Colorful terminal output (ANSI colors)
- [x] Configuration file support (.katana)
- [x] Environment variables support
- [x] Directory traversal protection
- [x] Hidden files filtering
- [x] Logging system with levels
- [x] HTTP Methods (GET, HEAD, OPTIONS, TRACE)
- [x] Port availability check

### Planned
- [ ] Help command support
- [ ] HTTPS/TLS support
- [ ] HTTP/2 support
- [ ] Compression (gzip, brotli)
- [ ] Custom error pages
- [ ] Access control (basic auth)
- [ ] Request rate limiting
- [ ] WebSocket support
- [ ] CGI/FastCGI support
- [ ] Plugin system

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Author

**Judicaël AHYI** ([@ludndev](https://github.com/ludndev))

- GitHub: [@ludndev](https://github.com/ludndev)
- Project: [ideatopia/katana](https://github.com/ideatopia/katana)

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Special thanks to [@tobihans](https://github.com/tobihans) for his invaluable feedbacks and support
- Thanks to the tech communities ([@PythonBenin](https://github.com/PythonBenin) and [@JsBenin](https://github.com/jsbenin)) for their encouragement
- Thanks to all contributors and users!

## Support

- **Issues**: [GitHub Issues](https://github.com/ideatopia/katana/issues)
- **Discussions**: [GitHub Discussions](https://github.com/ideatopia/katana/discussions)

---

<div align="center">

**Star this repository if you find it useful!**

Made with ❤️ and Rust

</div>