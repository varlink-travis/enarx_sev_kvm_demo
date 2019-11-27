## Current State
* Sets up kvm in x86 64bit mode with pagetables
* Boots to a modified [blog_os kernel](https://os.phil-opp.com/)
* Exception handling
* Serial print
* Exit codes

## TODO
### vmrun
* Lots of refactoring!
* Use other crates:
    * https://github.com/firecracker-microvm/firecracker
    * https://github.com/rust-vmm
    * https://github.com/cloud-hypervisor/rust-hypervisor-firmware
    * https://github.com/rust-osdev/x86_64
### kernel    
* Memory management via mmap() proxying to vmrun
* Start elf binary in Ring 3
* Handle syscalls
* Thread creation via clone() in vmrun
* Maybe use [mimalloc](https://github.com/microsoft/mimalloc) as [allocator](https://github.com/purpleprotocol/mimalloc_rust) 

## Requirements

```bash
$ rustup toolchain add nightly
$ rustup toolchain add nightly-2019-11-17
$ rustup component add rust-src
$ cargo install cargo-xbuild
```

*Note*: [`nightly-2019-11-17` has `clippy`](https://rust-lang.github.io/rustup-components-history/index.html)

## Run

```bash
$ cargo build --package vmrun
$ cargo +nightly xrun --package kernel --target kernel/x86_64-kernel.json
```

or

```bash
$ cargo +nightly xbuild --package kernel --target kernel/x86_64-kernel.json
$ cargo run --package vmrun --bin vmrun -- target/x86_64-kernel/debug/kernel
```

## Test

```bash
$ cargo build --package vmrun
$ cargo +nightly xtest --package kernel --target kernel/x86_64-kernel.json
```

## Clippy

```bash
$ cargo clean
$ cargo clippy
$ cargo +nightly-2019-11-17 xclippy --package kernel --target kernel/x86_64-kernel.json
```