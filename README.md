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

3. **Compile:**
    - Debug
    ```bash
    cargo +nightly build --target riscv64gc-unknown-linux-musl -Zbuild-std
    ```
    - Release
    ```bash
    cargo +nightly build --target riscv64gc-unknown-linux-musl -Zbuild-std --release
    ```

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

## Troubleshooting
If you have trouble running the binary on the milk-v duo, the article mentions you can get by using:
```bash
ln -sf /lib/ld-musl-riscv64v0p7_xthead.so.1 /lib/ld-musl-riscv64.so.1
```
However, I would still get an error:
```bash
[root@milkv-duo]~# ./milkv-duo-rust 
Error relocating ./milkv-duo-rust: pthread_getname_np: symbol not found
Error relocating ./milkv-duo-rust: pthread_getname_np: symbol not found
```
I was able to get away with copying `libc.so` from the toolchain onto the milk-v duo.
```bash
scp ./riscv64-lp64d--musl--bleeding-edge-2023.11-1/riscv64-buildroot-linux-musl/sysroot/lib/libc.so root@192.168.42.1:/lib/ld-musl-riscv64.so.1
```

```bash
[root@milkv-duo]~# ./milkv-duo-rust 
Hello, world!
1970-01-01T00:47:05.638969Z  INFO milkv_duo_rust: Rust is the future
```
