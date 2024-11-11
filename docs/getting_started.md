# Getting Started with EVM-Indexer

Welcome to EVM-Indexer! This guide will help you get started with installing and running the indexer.

## Installation Options

**WARNING**: At the moment, the indexer is only tested on Linux x86_64. Other platforms may not work as expected.

To begin, you will need to clone the repository:

```bash
git clone https://github.com/mrzigha/evm-indexer --recursive --branch [TAG] evm-indexer
cd evm-indexer
```

Choose one of the following installation methods:

### 1. Docker Installation (Recommended)
For users who prefer containerized applications, follow our [Docker Installation Guide](installation/install_docker.md).

This method is recommended if you:
- Want a simplified setup process
- Need easy deployment and scaling
- Prefer isolated environments

### 2. Binary Installation
For users who want to run the indexer directly on their system, follow our [Binary Installation Guide](installation/install_binary.md).

This method is recommended if you:
- Want direct system access
- Need maximum performance
- Are comfortable with Rust and system dependencies
- Want to customize the build

## Next Steps

After installation, you'll need to:
1. Configure your environment variables
2. Set up your MongoDB instance
3. Prepare your contract ABI
4. Configure your RPC endpoints

These steps are covered in detail in both installation guides.

## Need Help?

If you encounter any issues:
- Check our [troubleshooting guide](troubleshooting.md)
- Open an issue on GitHub
- Contact me directly on Discord (username: `@mrzigha`)

## Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md) for more information.
