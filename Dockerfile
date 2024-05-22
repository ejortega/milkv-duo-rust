FROM rustlang/rust:nightly-slim

ARG TARGET=riscv64gc-unknown-linux-gnu

RUN apt-get update \
    && apt-get upgrade -y \
    && apt-get install -y \
    build-essential \
    wget \
    gosu \
    # Perform cleanup
    && apt-get autoclean \
    && apt-get autoremove \
    && rm -rf /var/lib/apt/lists/*

# Add the target to the rust toolchain
RUN rustup +nightly target add $TARGET \
    && rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu

RUN wget https://toolchains.bootlin.com/downloads/releases/toolchains/riscv64-lp64d/tarballs/riscv64-lp64d--musl--bleeding-edge-2023.11-1.tar.bz2 \
    && tar xvf riscv64-lp64d--musl--bleeding-edge-2023.11-1.tar.bz2 \
    && rm riscv64-lp64d--musl--bleeding-edge-2023.11-1.tar.bz2

WORKDIR /app

# Copy the entrypoint script
COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]
