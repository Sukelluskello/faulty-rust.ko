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

 1. Install the prerequisites (Debian/Ubuntu [1])
```
$ sudo apt install -y linux-headers-`uname r` make curl gcc libclang-dev
```

 2. Install the nightly Rust [2]:
```
$ rustup install nightly
$ rustup default nightly
```
 3. Install xargo and rust-src
```
$ cargo install xargo and rust-src
$ sudo ln -s ~/.cargo/bin/xargo /usr/bin/xargo
$ rustup component add rust-src
```
 4. Build the module
```
$ make
$ sudo insmod rfaulty.ko
```

# Building with Vagrant

If you have Virtualbox [3] and Vagrant you can just
```
$ vagrant up
$ vagrant ssh
$ cd /vagrant
$ make
$ sudo insmod rfaulty.ko
```

[1] I got this working only on Ubuntu 18.10 (Cosmic), Debian 9 (Stretch) didn't work. Buster pre-release also had some problems.

[2] Tested on versions 1.35.0-nightly (f69422288 2019-04-01) and rustc 1.35.0-nightly (3750348da 2019-04-08)

[3] Unfortunately, the Ubuntu 18.10 box is provided only for Virtualbox, not for Libvirt.
