FROM rust:1.82 AS builder

WORKDIR /usr/src/app
COPY . .

RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && \
    apt-get install -y libssl3 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/evm-indexer /app/evm-indexer

EXPOSE 9090

ENTRYPOINT [ "/app/evm-indexer" ]
