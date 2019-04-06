# Faulty kernel module in Rust

This is a work based on https://github.com/tsgates/rust.ko . Its
purpose is to try if some typical c programming errors translate to
Rust as well. See https://github.com/isido/kernel-module-with-faults

# Divergences from the original module

 - removed --gc-sections to fix link errors (not sure what is the root
   cause)
 - changed the C-code (my original work) license to GPL for easier
   dealings with kernel symbols (module.c is GPL licensed, otherwise
   the code retains the original MIT license)

# Building

 1. Install nightly Rust:
```
$ rustup install nightly
$ rustup default nightly
```
 2. Install xargo and rust-src
```
$ cargo install xargo
```
 3. Optionally symlink it
```
$ sudo ln -s ~/.cargo/bin/xargo /usr/bin/xargo
$ rustup component add rust-src
```
 4. Build the module
```
$ make
$ sudo insmod rfaulty.ko
```

