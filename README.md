# Compiling Rust Projects for Milk-V Duo

This guide provides instructions on how to compile Rust projects for the Milk-V Duo, a device using the RISC-V architecture.

This is inspired from reading <https://barretts.club/posts/i-got-a-milkv-duo/>.

Rust toolchain for riscv64gc-unknown-linux-musl is now Tier 2

## Prerequisites for building with Docker

- Docker installed

## Docker Instructions

### Using the Prebuilt Docker Image

This is the easiest way to build as you can support all operating systems that have a docker installation and using x86_64 or aarch64 processors.

1. **Build Debug Version:**

   ```bash
   docker run --rm -e LOCAL_UID=$(id -u) -e LOCAL_GID=$(id -g) -v $PWD:/app ejortega/duo-rust:2.0 cargo build --target riscv64gc-unknown-linux-musl
   ```

   or use the provided python script

   ```bash
   ./build.py
   ```

2. **Build Release Version:**

   ```bash
   docker run --rm -e LOCAL_UID=$(id -u) -e LOCAL_GID=$(id -g) -v $PWD:/app ejortega/duo-rust:2.0 cargo build --target riscv64gc-unknown-linux-musl --release
   ```

   or

   ```bash
   ./build.py --release
   ```

### Build and Run Your Own Image

1. **Build the Docker Image:**
   Replace `<tag>` with your preferred tag name.

   ```bash
   docker build -t <tag> .
   ```

2. **Compile Your Application:**
   Use the following command to compile your app. Replace `<tag>` with the tag used above.

   ```bash
   docker run --rm -e LOCAL_UID=$(id -u) -e LOCAL_GID=$(id -g) -v $PWD:/app <tag> cargo build --target riscv64gc-unknown-linux-musl --release
   ```

## Prerequisites without using Docker

- Linux operating system (x86_64 or aarch64)
- Rust programming environment

## Setup Instructions

1. **Add RISC-V Target in Rust:**
   Run the following command to add the `riscv64gc-unknown-linux-musl` target.

   ```bash
   rustup target add riscv64gc-unknown-linux-musl
   ```

2. **Download a risv64 musl toolchain:**

   <https://github.com/ejortega/milkv-host-tools/releases/tag/v2.0.0>

   Alternatives:
      - <https://toolchains.bootlin.com/releases_riscv64.html>
      - <https://github.com/riscv-collab/riscv-gnu-toolchain>

   Ensure the path to the linker in is updated in the `config/cargo.toml`.

   You might get away with installing `gcc-riscv64-linux-gnu` via:

   ```bash
   sudo apt install gcc-riscv64-linux-gnu
   ```

   and updating the linker with `linker = "riscv64-linux-gnu-gcc"`. However since the milkv duo uses musl, you might have compatability issues.

3. **Compile:**

   Update `.config/cargo.toml`.

   - Compile Debug

   ```bash
   cargo build --target riscv64gc-unknown-linux-musl
   ```

   - Compile Release

   ```bash
   cargo build --target riscv64gc-unknown-linux-musl --release
   ```

4. **Testing:**

   Copy the `hello-world` binary to your duo (assuming release build):

   ```bash
   scp target/riscv64gc-unknown-linux-musl/release/hello-world root@192.168.42.1:/root/
   ```

   You may need `-O` for more recent versions of `scp`

   ```bash
   scp -O target/riscv64gc-unknown-linux-musl/release/hello-world root@192.168.42.1:/root/
   ```

   Update permission for the binary

   ```bash
   [root@milkv-duo]~# chmod +x hello-world 
   ```

   ```bash
   [root@milkv-duo]~# ./hello-world 
   Hello, world!
   1970-01-01T00:47:05.638969Z  INFO milkv_duo_rust: Rust is the future
   ```
