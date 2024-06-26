FROM rust:slim-bookworm

ARG TARGET=riscv64gc-unknown-linux-gnu
ARG TARGETPLATFORM
ARG USER_ID=1000
ARG GROUP_ID=1000

RUN apt-get update \
    && apt-get upgrade -y \
    && apt-get install -y \
    build-essential \
    curl \
    # Perform cleanup
    && apt-get autoclean \
    && apt-get autoremove \
    && rm -rf /var/lib/apt/lists/*

# Add the target to the rust toolchain
RUN rustup +nightly target add $TARGET && \
    if [ "$TARGETPLATFORM" = "linux/amd64" ]; then \
        rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu; \
    elif [ "$TARGETPLATFORM" = "linux/arm64" ]; then \
        rustup component add rust-src --toolchain nightly-aarch64-unknown-linux-gnu; \
    fi

RUN FILE_URL="" && \
    if [ "$TARGETPLATFORM" = "linux/amd64" ]; then \
        FILE_URL="https://github.com/ejortega/milkv-host-tools/releases/download/v1.0.0/toolchain-riscv64-unknown-linux-musl-amd64.tar.xz"; \
    elif [ "$TARGETPLATFORM" = "linux/arm64" ]; then \
        FILE_URL="https://github.com/ejortega/milkv-host-tools/releases/download/v1.0.0/toolchain-riscv64-unknown-linux-musl-arm64.tar.xz"; \
    fi \
    && curl -L -o toolchain.tar.xz $FILE_URL \
    && tar -xf toolchain.tar.xz \
    && rm toolchain.tar.xz

RUN groupadd -g $GROUP_ID user && \
    useradd -m -s /bin/bash -u $USER_ID -g $GROUP_ID user

RUN chown -R user:user /toolchain-riscv64-unknown-linux-musl

# Copy the entrypoint script
COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

WORKDIR /app
ENTRYPOINT ["/entrypoint.sh"]
USER user
