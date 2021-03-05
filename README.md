WAVM
====

WAVM, *Wait, another virtual machine ?*, is a register based 64 bits virtual machine written in Rust.

If you are searching for the Web Assembly Virtual Machine, go [here](https://github.com/wavm/wavm)

It relies on 32 registers and 31 opcodes that permit to do various things.

It contains both a compiler to build bytecode for assembly and a virtual machine to run the produced bytecode.

CI
--

[![Build-test](https://github.com/Wafelack/wavm/actions/workflows/rust.yml/badge.svg)](https://github.com/Wafelack/wavm/actions/workflows/rust.yml)

Installation
------------

You can either:

* Build from source: 

```bash
$ git clone git@github.com:Wafelack/wavm.git
$ cd wavm
$ cargo test
$ cargo build --release
```

* Install from crates.io: `cargo install wavm-cli && mv ~/.cargo/bin/wavm-cli ~/.cargo/bin/wavm`.

Example
-------

Staying classing, here an Hello, World !

```asm
ascii %0 'Hello, World !'
dsp %0
```

Documentation
-------------

Documentation is available in [the docs folder](./docs/src/) or on [the website](https://wafelack.fr/wavm).

Licensing
---------

WAVM is licensed under the GNU General Public License version 3.0.
