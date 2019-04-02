# Faulty kernel module in Rust

This is a work based on https://github.com/tsgates/rust.ko . Its
purpose is to try if some typical c programming errors translate to
Rust as well. See https://github.com/isido/kernel-module-with-faults

# Building

 1. Install nightly rust
  ```$ rustup install nightly```
  ```$ rustup default nightly```
  
 2. Install xargo and rust-src
  ```$ cargo install xargo```
  optionally symlink it
  ```$ sudo ln -s ~/.cargo/bin/xargo /usr/bin/xargo```
  ```$ rustup component add rust-src```
  
 3. Build the module
  ```make```
