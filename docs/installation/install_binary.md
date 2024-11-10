# Binary Installation Guide

## Prerequisites

Before you begin, make sure you have the following installed:
* [Rust](https://www.rust-lang.org/tools/install) (stable >= 1.82)
* pkg-config for your OS (e.g. `pkg-config` for Ubuntu)
* Lib SSL for your OS (e.g. `libssl-dev` for Ubuntu)

## Building the Binary

Assuming you have the prerequisites installed and already cloned the repository, follow these steps:

To build the binary, run the following command:

```bash
cargo build --release
```
By default, the binary will be built in the `target/release` directory.

Next copy the binary to the root of the repository:

```bash
cp target/release/evm-indexer .
```

## Configure the Environment and Configuration Files

First, create folders for the logs and configuration files:

```bash
mkdir logs
mkdir config
```

Copy examples configuration files to the root directory:

```bash
cp example/.env.example config/.env
cp example/config.toml.example config/config.toml
```

Edit the `.env` and `config.toml` files to match your environment.

## Running the Binary

To run the binary, use the following command:

```bash
source config/.env && ./evm-indexer
``` 
