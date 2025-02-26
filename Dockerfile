FROM rust:slim-bookworm

ARG TARGET=riscv64gc-unknown-linux-musl
ARG USER_ID=1000
ARG GROUP_ID=1000

RUN apt-get update \
    && apt-get install -y \
    gcc-riscv64-linux-gnu \
    # Perform cleanup
    && apt-get autoclean \
    && apt-get autoremove \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add $TARGET

RUN groupadd -g $GROUP_ID user && \
    useradd -m -s /bin/bash -u $USER_ID -g $GROUP_ID user

COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

USER user
WORKDIR /app

ENTRYPOINT ["/entrypoint.sh"]
