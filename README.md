# Rust-Z80

A fork of the Rust compiler with Z80 and SM83 (Game Boy) target support,
using the [llvm-z80](https://github.com/llvm-z80/llvm-z80) LLVM backend.

## Supported targets

- `z80-unknown-none-elf`
- `sm83-nintendo-none-elf`

## Building

1. Build the llvm-z80 LLVM backend:

```sh
git clone https://github.com/llvm-z80/llvm-z80.git
cd llvm-z80
cmake -S llvm -B build -G Ninja \
  -DCMAKE_BUILD_TYPE=Release \
  -DLLVM_TARGETS_TO_BUILD="Z80"
ninja -C build
```

2. Create `bootstrap.toml` in the Rust source root:

```toml
[llvm]
download-ci-llvm = false

[target.x86_64-unknown-linux-gnu]
llvm-config = "/path/to/llvm-z80/build/bin/llvm-config"
llvm-has-rust-patches = false
```

3. Build the compiler:

```sh
./x.py build --target z80-unknown-none-elf library
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
