# Docker Installation Guide

## Prerequisites

Before you begin, make sure you have the following installed:
* [Docker](https://docs.docker.com/get-docker/)
* [Docker Compose](https://docs.docker.com/compose/install/)
* [Docker BuildKit](https://docs.docker.com/build/buildkit/)

## Building the Docker Image

Assuming you have the prerequisites installed and already cloned the repository, follow these steps:

To build the Docker image, run the following command:

```bash
docker buildx build -t evm-indexer:[TAG] .
```

Replace `[TAG]` with the version you want to build.

## Configure the Environment and Configuration Files

Copy examples configuration files to the root directory:

```bash
cp example/.env.example .env
cp example/config.toml.example config.toml
```
Edit the `.env` and `config.toml` files to match your environment.

## Edit the Docker Compose File

Replace `[TAG]` with the version you want to build in the `docker-compose.yml` file.
Replace `[YOUR LOGS FOLDER]` with the path to your logs folder.
Replace `[YOUR CONFIG FILE]` with the path to your config file.
Replace `[YOUR ABI FILE]` with the path to your abi file.

## Running the Docker Container

To run the Docker container, use the following command:

```bash
docker compose up -d
```

To stop the Docker container, use the following command:

```bash
docker compose down
```
