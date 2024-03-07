FROM rustlang/rust:nightly-slim

ARG TARGET=riscv64gc-unknown-linux-gnu

RUN apt-get update \
    && apt-get install -y \
    build-essential \
    wget \
    gosu \
    # Clean up
    && rm -rf /var/lib/apt/lists/*

# Add the target to the rust toolchain
RUN rustup +nightly target add $TARGET
RUN rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu

WORKDIR /app

# Copy the entrypoint script
COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]
