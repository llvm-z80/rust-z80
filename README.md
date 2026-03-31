# Rust-Z80

A fork of the Rust compiler with Z80 and SM83 (Game Boy) target support,
using the [llvm-z80](https://github.com/llvm-z80/llvm-z80) LLVM backend.

## Supported targets

- `z80-unknown-none-elf`
- `sm83-nintendo-none-elf`

## Building

### Prerequisites

Build [llvm-z80](https://github.com/llvm-z80/llvm-z80) first and note the path to `llvm-config`.

### 1. Configure `bootstrap.toml`

Adjust paths and targets to match your environment.

```toml
[rust]
deny-warnings = false

[llvm]
download-ci-llvm = false

[build]
target = ["x86_64-unknown-linux-gnu", "z80-unknown-none-elf", "sm83-nintendo-none-elf"]
extended = true
tools = ["cargo", "rust-analyzer-proc-macro-srv"]

[target.x86_64-unknown-linux-gnu]
llvm-config = "/path/to/llvm-z80/build/bin/llvm-config"
llvm-has-rust-patches = false

[target.z80-unknown-none-elf]
optimized-compiler-builtins = false

[target.sm83-nintendo-none-elf]
optimized-compiler-builtins = false
```

### 2. Build

```sh
./x build library
```

Builds the stage 1 compiler and `core`/`alloc`/`compiler_builtins` for all configured targets (`no_std` only).

If your project uses proc macros, build stage 2 instead.
The proc-macro-srv built at stage 1 has a different ABI than the stage 1 compiler,
so proc macro crates compiled by stage 1 rustc cannot be loaded.

```sh
./x build --stage 2
```

### 3. Link toolchain

```sh
rustup toolchain link rust-z80 build/x86_64-unknown-linux-gnu/stage1
# or stage2 if you need proc macros:
rustup toolchain link rust-z80 build/x86_64-unknown-linux-gnu/stage2
```

## New ABIs

- `extern "sdcccall-0"` — SDCC calling convention (feature gate: `abi_sdcccall0`)
- `extern "z80-interrupt"` — Z80 interrupt handler (feature gate: `abi_z80_interrupt`)

## License

Rust is primarily distributed under the terms of both the MIT license and the
Apache License (Version 2.0), with portions covered by various BSD-like
licenses.

See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT), and
[COPYRIGHT](COPYRIGHT) for details.
