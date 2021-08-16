# osh1mc

An OS written in Rust.

# How to run

Install QEMU and execute following commands.
```
$ rustup install nightly
$ rustup default nightly
$ rustup component add llvm-tools-preview
$ cargo install bootimage
$ cargo bootimage
$ cargo run
```
