# aya-telemetry

## Security Policy & Monitored Directories

This telemetry and enforcement tool monitors specific directories and applies actions based on the operation performed:

- **`/opt/protected`**: Highly sensitive directory. Any **Write** or **Delete** operations in this path will trigger a **Kill** action, terminating the offending process. All other operations (e.g., Open/Read) are logged.
- **`/var/secure`** and **`/home/secure_area`**: Monitored directories. All operations within these paths are **Logged**.
- Operations occurring outside of these specified directories are **Ignored**.

## Prerequisites

1. stable rust toolchains: `rustup toolchain install stable`
1. nightly rust toolchains: `rustup toolchain install nightly --component rust-src`
1. (if cross-compiling) rustup target: `rustup target add ${ARCH}-unknown-linux-musl`
1. (if cross-compiling) LLVM: (e.g.) `brew install llvm` (on macOS)
1. (if cross-compiling) C toolchain: (e.g.) [`brew install filosottile/musl-cross/musl-cross`](https://github.com/FiloSottile/homebrew-musl-cross) (on macOS)
1. bpf-linker: `cargo install bpf-linker` (`--no-default-features` on macOS)

## Fast Run

To quickly build the eBPF component and run the userspace application with sudo and info logging, use the following commands:

```shell
cd aya-telemetry-ebpf
cargo +nightly build --target bpfel-unknown-none -Z build-std=core --release
cd ..
cd aya-telemetry
sudo RUST_LOG=info cargo run --release
```

## Testing

To run the unit tests for the userspace application, use the following commands:

```shell
cd aya-telemetry
cargo test
```

## Build & Run

Use `cargo build`, `cargo check`, etc. as normal. Run your program with:
```shell
cargo run --release

Cargo build scripts are used to automatically build the eBPF correctly and include it in the
program.
```
## Cross-compiling on macOS

Cross compilation should work on both Intel and Apple Silicon Macs.

```shell
CC=${ARCH}-linux-musl-gcc cargo build --package aya-telemetry --release \
  --target=${ARCH}-unknown-linux-musl \
  --config=target.${ARCH}-unknown-linux-musl.linker=\"${ARCH}-linux-musl-gcc\"
The cross-compiled program `target/${ARCH}-unknown-linux-musl/release/aya-telemetry` can be
copied to a Linux server or VM and run there.
```

## License

With the exception of eBPF code, aya-telemetry is distributed under the terms
of either the [MIT license] or the [Apache License] (version 2.0), at your
option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

### eBPF

All eBPF code is distributed under either the terms of the
[GNU General Public License, Version 2] or the [MIT license], at your
option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the GPL-2 license, shall be
dual licensed as above, without any additional terms or conditions.

[Apache license]: LICENSE-APACHE
[MIT license]: LICENSE-MIT
[GNU General Public License, Version 2]: LICENSE-GPL2
