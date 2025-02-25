# Resource Monitor & Analytics System

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-blue?logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-orange)](LICENSE)

A distributed system for collecting, processing, and visualizing system metrics, built with Rust's safety and concurrency features.

## 📋 Overview

![System Architecture](docs/architecture.png)  
*(Example architecture diagram - create actual diagram with mermaid.js/draw.io)*

A two-component system consisting of:
1. **Monitor Agent** (CLI) - Collects and sends system metrics
2. **Analytics Server** - Multi-threaded HTTP server with REST API for processing/storing metrics

## ✨ Features

- Real-time monitoring of CPU, memory, disk, and network usage
- Multi-threaded metric processing pipeline
- REST API for data access and configuration
- Secure JWT authentication
- PostgreSQL time-series storage
- Configurable collection intervals
- Cross-platform support (Linux/macOS/Windows)

## 🛠 Tech Stack

| Component              | Crates/Tools                                                                 |
|------------------------|------------------------------------------------------------------------------|
| **Monitor Agent**      | `sysinfo`, `reqwest`, `tokio`, `clap`                                       |
| **Analytics Server**   | `actix-web`, `sqlx`, `tokio`, `serde`, `crossbeam-channel`, `chrono`         |
| **Data Processing**    | `rayon`, `ndarray`, `statrs`                                                |
| **Observability**      | `tracing`, `tracing-subscriber`, `prometheus`                               |
| **Persistence**        | PostgreSQL, `sqlx`, `time-series`                                           |

## 🚀 Getting Started

### Prerequisites
- Rust 1.70+
- PostgreSQL 14+
- Tokio runtime

### Installation

```bash
# Clone repository
git clone https://github.com/yourusername/resource-monitor-system.git
cd resource-monitor-system

# Build both components
cargo build --release

🖥 Usage
Start Analytics Server
bash
Copy

# Set up environment variables
cp .env.example .env
# Edit .env with your database credentials

# Run server
cargo run --bin server --release

Start Monitor Agent
bash
Copy

# Configure target server and collection interval
cargo run --bin agent --release -- \
  --server-url http://localhost:8080 \
  --interval 5

🌐 API Endpoints
Endpoint	Method	Description
/api/v1/metrics	POST	Submit new metrics
/api/v1/stats	GET	Get aggregated statistics
/api/v1/history	GET	Retrieve historical data
/api/v1/alerts	GET	List active alerts

Example Request:
bash
Copy

curl -X POST http://localhost:8080/api/v1/metrics \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -d '{
    "cpu_usage": 45.2,
    "memory_used": 8192,
    "timestamp": "2023-07-20T12:34:56Z"
  }'

⚙ Configuration

Environment variables (.env file):
ini
Copy

DATABASE_URL=postgres://user:pass@localhost/resmon
JWT_SECRET=your_jwt_secret_key
MAX_WORKERS=8
BUFFER_SIZE=1000

🔒 Safety & Performance

    Memory Safety: Leverages Rust's ownership system

    Concurrency: Thread pools and async I/O

    Rate Limiting: 1000 req/minute default

    TLS: Supports HTTPS through rustls

    Validation: Strict metric schema validation

🤝 Contributing

    Fork the repository

    Create feature branch (git checkout -b feature/foo)

    Commit changes (git commit -am 'Add foo')

    Push to branch (git push origin feature/foo)

    Create new Pull Request

📄 License

MIT License - see LICENSE file for details

Acknowledgments: Thanks to the Rust community and crate maintainers!
