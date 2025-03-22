FROM rust:slim-bookworm

ARG TARGET=riscv64gc-unknown-linux-musl
ARG USER_ID=1000
ARG GROUP_ID=1000

ARG TARGETPLATFORM

RUN apt-get update \
    && apt-get install -y \
    curl xz-utils \
    # Perform cleanup
    && apt-get autoclean \
    && apt-get autoremove \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add $TARGET

RUN groupadd -g $GROUP_ID user && \
    useradd -m -s /bin/bash -u $USER_ID -g $GROUP_ID user

COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

RUN FILE_URL="" && \
    if [ "$TARGETPLATFORM" = "linux/amd64" ]; then \
    FILE_URL="https://github.com/ejortega/milkv-host-tools/releases/download/v2.0.0/riscv64-unknown-linux-musl-amd64.tar.xz"; \
    elif [ "$TARGETPLATFORM" = "linux/arm64" ]; then \
    FILE_URL="https://github.com/ejortega/milkv-host-tools/releases/download/v2.0.0/riscv64-unknown-linux-musl-arm64.tar.xz"; \
    fi \
    && curl -L -o toolchain.tar.xz $FILE_URL \
    && tar -xf toolchain.tar.xz \
    && rm toolchain.tar.xz

USER user
WORKDIR /app

ENV PATH=/riscv64-unknown-linux-musl/bin:$PATH

ENTRYPOINT ["/entrypoint.sh"]
