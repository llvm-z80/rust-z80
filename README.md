# Rust-Z80

A fork of the Rust compiler with Z80 and SM83 (Game Boy) target support,
using the [llvm-z80](https://github.com/llvm-z80/llvm-z80) LLVM backend.

## Supported targets

- `z80-unknown-none-elf`
- `sm83-nintendo-none-elf`

## Building

`rust-z80` bundles its LLVM backend as the `src/llvm-project` submodule (the
[llvm-z80](https://github.com/llvm-z80/llvm-z80) fork) and builds it in-tree, so
the installed toolchain is self-contained: `rustc`, `cargo`, the SM83 `ld.lld`,
and the `Z80` LLVM backend ship together.

### 1. Clone with the LLVM submodule

```sh
git clone https://github.com/zlfn/rust rust-z80
cd rust-z80
git submodule update --init --depth 1 src/llvm-project
```

### 2. Configuration

The repo ships a working [`bootstrap.toml`](bootstrap.toml): it builds LLVM
in-tree with the `X86` and `Z80` backends (the `Z80` backend also covers SM83),
bundles `lld`, and installs `cargo`. Adjust it only if your setup differs.

### 3. Build and install

```sh
./x install
```

Builds the stage 2 compiler (the first run also compiles LLVM, which is slow)
and installs `rustc`, `cargo`, `lld`, and the `core`/`alloc` for the Z80 and
SM83 targets into `install/`.

### 4. Register the toolchain

```sh
rustup toolchain link rust-z80 install
```

`install/` holds the compiler, cargo, linker, and target `std` together, so
`cargo +rust-z80 build` works with no further configuration.

## New ABIs

- `extern "sdcccall-0"` — SDCC calling convention (feature gate: `abi_sdcccall_0`)
- `extern "z80-interrupt"` — Z80 interrupt handler (feature gate: `abi_z80_interrupt`)

## License

Rust is primarily distributed under the terms of both the MIT license and the
Apache License (Version 2.0), with portions covered by various BSD-like
licenses.

See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT), and
[COPYRIGHT](COPYRIGHT) for details.
