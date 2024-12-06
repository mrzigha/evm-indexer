# EVM-Indexer 🚀

<div align="center">

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.82+-orange.svg)
![Status](https://img.shields.io/badge/status-production_ready-green.svg)
[![Release](https://github.com/mrzigha/evm-indexer/actions/workflows/release.yml/badge.svg?branch=main)](https://github.com/mrzigha/evm-indexer/actions/workflows/release.yml)

A high-performance, fault-tolerant Web3 event indexer written in Rust. This tool efficiently indexes smart contract events across multiple EVM-compatible blockchains and stores them in MongoDB.

[Getting Started](docs/getting_started.md) •
[Installation](docs/installation/) •
[Documentation](docs/) •
[Contributing](CONTRIBUTING.md)

</div>

## ✨ Key Features

- **🔗 Multi-Chain Support**
  - Index events from any EVM-compatible blockchain
  - Support for multiple chains simultaneously
  - Flexible chain configuration

- **🛡️ Fault Tolerance**
  - Circuit breaker pattern implementation
  - Automatic RPC endpoint failover
  - Reconnection handling with exponential backoff
  - Error recovery mechanisms

- **⚡ Flexible Sync Modes**
  - Real-time event monitoring via WebSocket or HTTP(S)
  - Historical synchronization via HTTP(S)
  - Parallel processing capabilities
  - Intelligent polling for HTTP endpoints

- **📊 Efficient Data Management**
  - MongoDB integration for reliable storage
  - Duplicate event detection
  - Batch processing for historical sync
  - Automatic event decoding via ABI

- **📈 Monitoring & Observability**
  - Prometheus metrics
  - Health check endpoints
  - Comprehensive logging
  - Performance metrics tracking

- **🐳 Docker Integration**
  - Complete Docker support
  - Docker Compose setup included
  - Volume mounting for configuration
  - Easy deployment and scaling

## 🚀 Production Ready

- ✅ Battle-tested in production environments
- 🔄 Handles network instability gracefully
- 📊 Efficient resource utilization
- 🛡️ Comprehensive error handling
- 🎯 Clean shutdown mechanisms
- 🔄 Support for both WebSocket and HTTP(S) monitoring

## 🛠️ Built With

- [Rust](https://www.rust-lang.org/) - For performance and reliability
- [Web3](https://docs.rs/web3) - Ethereum interface
- [MongoDB](https://www.mongodb.com/) - Event storage
- [Prometheus](https://prometheus.io/) - Metrics and monitoring
- [Docker](https://www.docker.com/) - Containerization


## 📊 Project Status

🟢 Active Development | 🔄 Regular Updates | ⚡ Production Ready

## 📖 Documentation

Comprehensive installation guide is available in the [Getting started](./docs/getting_started.md) section.

## 🤝 Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md) before submitting a Pull Request.

## ⚖️ License

This project is licensed under the MIT License - see the [LICENSE](LICENSE.md) file for details.

## 🆘 Support

For issues and feature requests, please [open an issue](https://github.com/mrzigha/evm-indexer/issues).

## ☕ Buy me a Coffee

If you find this project useful, consider supporting its development:

ETH: 0xc7752c0254d5b4cc3ab8ec497045e8de2e4c901e