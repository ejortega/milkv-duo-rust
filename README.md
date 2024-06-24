# Compiling Rust Projects for Milk-V Duo

This guide provides instructions on how to compile Rust projects for the Milk-V Duo, a device using the RISC-V architecture.

This is inspired from reading https://barretts.club/posts/i-got-a-milkv-duo/.

## Prerequisites for building with Docker

- Docker installed

## Docker Instructions

### Using the Prebuilt Docker Image

This is the easiest way to build as you can support all operating systems that have a docker installation and using x86_64 or aarch64 processors.

1. **Build Debug Version:**

   ```bash
   docker run --rm -e LOCAL_UID=$(id -u) -e LOCAL_GID=$(id -g) -v $PWD:/app ejortega/duo-rust cargo +nightly build --target riscv64gc-unknown-linux-musl -Zbuild-std
   ```

   or use the provided python script

   ```bash
   ./build.py
   ```

2. **Build Release Version:**
   ```bash
   docker run --rm -e LOCAL_UID=$(id -u) -e LOCAL_GID=$(id -g) -v $PWD:/app ejortega/duo-rust cargo +nightly build --target riscv64gc-unknown-linux-musl -Zbuild-std --release
   ```
   or
   ```
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
   docker run --rm -e LOCAL_UID=$(id -u) -e LOCAL_GID=$(id -g) -v $PWD:/app <tag> cargo +nightly build --target riscv64gc-unknown-linux-musl -Zbuild-std --release
   ```

## Prerequisites without using Docker

- Linux operating system (x86_64 or aarch64)
- Rust programming environment

## Setup Instructions

1. **Add RISC-V Target in Rust:**
   Run the following command to add the `riscv64gc-unknown-linux-gnu` target.

   ```bash
   rustup +nightly target add riscv64gc-unknown-linux-gnu
   ```

2. **Download and Extract Toolchain:**

   - I've provided compiled toolchains for x86_64 and aarch64. Download the appropriate RISC-V toolchain from [this link](https://github.com/ejortega/milkv-host-tools).
   - Extract it in your project directory:
      ```bash
      tar xvf toolchain-riscv64-unknown-linux-musl-amd64.tar.xz 
      ```
     or (depending on arch)

      ```bash
      tar xvf toolchain-riscv64-unknown-linux-musl-arm64.tar.xz 
      ```
   - Update `.config/config.toml` in your project if you use a different toolchain version or to update the linker and sysroot paths.

3. **Compile:**

   First make sure you the update the `linker` and `sysroot` paths in the `.config/cargo.toml`.

   - Compile Debug

   ```bash
   cargo +nightly build --target riscv64gc-unknown-linux-musl -Zbuild-std
   ```

   - Compile Release

   ```bash
   cargo +nightly build --target riscv64gc-unknown-linux-musl -Zbuild-std --release
   ```

## Troubleshooting
*This should no longer be an issue after adding*
`"-C", "link-arg=-Wl,--dynamic-linker=/lib/ld-musl-riscv64v0p7_xthead.so.1"` to the config.toml but I'll leave for reference.

If you have trouble running the binary on the milk-v duo, the article mentions you can get by using:

```bash
ln -sf /lib/ld-musl-riscv64v0p7_xthead.so.1 /lib/ld-musl-riscv64.so.1
```

However, I would still get an error:

```bash
[root@milkv-duo]~# ./hello-world 
Error relocating ./hello-world: pthread_getname_np: symbol not found
Error relocating ./hello-world: pthread_getname_np: symbol not found
```

I was able to get away with copying `libc.so` from the toolchain onto the milk-v duo.

```bash
scp ./riscv64-lp64d--musl--bleeding-edge-2023.11-1/riscv64-buildroot-linux-musl/sysroot/lib/libc.so root@192.168.42.1:/lib/ld-musl-riscv64.so.1
```

```bash
[root@milkv-duo]~# ./hello-world 
Hello, world!
1970-01-01T00:47:05.638969Z  INFO milkv_duo_rust: Rust is the future
```
