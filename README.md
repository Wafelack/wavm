# WAVM

WAVM, *Wait, another virtual machine ?*, is a register based 64 bits virtual machine written in Rust.

It relies on 32 registers and 31 opcodes that permit to do various things.

It contains both a compiler to build bytecode for assembly and a virtual machine to run the produced bytecode.

## Example

Staying classing, here an Hello, World !

```asm
ascii %0 'Hello, World !'
dsp %0
```

## Documentation

Documentation is available in [the docs folder](./docs/src/) or on [the website](https://wafelack.fr/wavm).

## Licensing

WAVM is licensed under the GNU General Public License version 3.0.
