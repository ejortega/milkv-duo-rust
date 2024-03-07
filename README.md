# Compiling Rust Projects for Milk-V Duo

This guide provides instructions on how to compile Rust projects for the Milk-V Duo, a device using the RISC-V architecture.

This is inspired from reading https://barretts.club/posts/i-got-a-milkv-duo/.

## Prerequisites

- Linux operating system (x86_64)
- Docker installed on your system
- Rust programming environment

## Setup Instructions

1. **Add RISC-V Target in Rust:**
   Run the following command to add the `riscv64gc-unknown-linux-gnu` target.
   ```bash
   rustup +nightly target add riscv64gc-unknown-linux-gnu
   ```

2. **Download and Extract Toolchain:**
   - Download the RISC-V toolchain from [this link](https://toolchains.bootlin.com/downloads/releases/toolchains/riscv64-lp64d/tarballs/riscv64-lp64d--musl--bleeding-edge-2023.11-1.tar.bz2).
   - Extract it in your project directory:
     ```bash
     tar xvf riscv64-lp64d--musl--bleeding-edge-2023.11-1.tar.bz2
     ```
   - Update `.config/config.toml` in your project if you use a different toolchain version.

## Docker Instructions

### Build and Run Your Own Image

1. **Build the Docker Image:**
   Replace `<tag>` with your preferred tag name.
   ```bash
   docker build -t <tag> .
   ```

2. **Compile Your Application:**
   Use the following command to compile your app. Replace `<tag>` with the tag used above.
   ```bash
   docker run --rm -e LOCAL_UID=$(id -u) -e LOCAL_GID=$(id -g) -v $PWD:/app <tag> cargo +nightly build --target riscv64gc-unknown-linux-musl -Zbuild-std --release
   ```

### Use Prebuilt Docker Image

1. **Build Debug Version:**
   ```bash
   docker run --rm -e LOCAL_UID=$(id -u) -e LOCAL_GID=$(id -g) -v $PWD:/app ejortega/duo-rust cargo +nightly build --target riscv64gc-unknown-linux-musl -Zbuild-std
   ```

2. **Build Release Version:**
   ```bash
   docker run --rm -e LOCAL_UID=$(id -u) -e LOCAL_GID=$(id -g) -v $PWD:/app ejortega/duo-rust cargo +nightly build --target riscv64gc-unknown-linux-musl -Zbuild-std --release
   ```
