# Introduction

WAVM is a 64 bits register based virtual machine with a few opcodes, written in Rust.

## Overview

### Registers

This virtual machine has 32 registers defined by the following table.

| Number |     Usage      |
|--------|----------------|
|  0..30 |  Multi purpose |
|   31   |  Equality flag |

### OpCodes

There are 31 opcodes available, defined by the following table.

| Name | Bytecode Value | Section |
|------|----------------|---------|
| LOAD |     0x00       | [Miscellanous](./misc.md) |
| MOVE |     0x01       | [Miscellanous](./misc.md) |
| HLT  |     0xCC       | [Miscellanous](./misc.md) |
| DRG  |     0x99       | [Miscellanous](./misc.md) |
| DSP  |     0xAA       | [Miscellanous](./misc.md) |
| ADD  |     0x02       |  [Arithmetic](./math.md)  |
| SUB  |     0x03       |  [Arithmetic](./math.md)   |
| MUL  |     0x04       |  [Arithmetic](./math.md)   |
| DIV  |     0x05       |  [Arithmetic](./math.md)   |
| MOD  |     0x06       |  [Arithmetic](./math.md)   |
|  EQ  |     0x07       |   [Boolean](./bool.md)    |
| GEQ  |     0x08       |   [Boolean](./bool.md)    |
|  GE  |     0x09       |   [Boolean](./bool.md)    |
| LEQ  |     0x0A       |   [Boolean](./bool.md)    |
|  LE  |     0x0B       |   [Boolean](./bool.md)    |
| NEQ  |     0x0C       |   [Boolean](./bool.md)    |
|  NOT |     0x0D       |   [Boolean](./bool.md)    |
|  OR  |     0x0E       |   [Boolean](./bool.md)    |
| AND  |     0x0F       |   [Boolean](./bool.md)    |
| XOR  |     0x10       |   [Boolean](./bool.md)    |
|  JMP |     0x11       |    [Jumps](./jumps.md)     |
|JMPEQ |     0x12       |    [Jumps](./jumps.md)     |
| RJMP |     0x13       |    [Jumps](./jumps.md)     |
| RQM  |     0x14       |    [Memory](./memory.md)    |
|ASCII |     0x15       |    [Memory](./memory.md)    |
| SETB |     0x16       |    [Memory](./memory.md)    |
| GETB |     0x17       |    [Memory](./memory.md)    |
| MMOV |     0x18       |    [Memory](./memory.md)    |
| MSET |     0x19       |    [Memory](./memory.md)    |
| FREE |     0x1A       |    [Memory](./memory.md)    |

### Numbers

WAVM supports 3 numbers types:

- Octal (`%` prefix).
- Hexadecimal (`0x` prefix).
- Binary (`0b` prefix).
- Decimal (No prefix).
- `$EQ` variable to index register 31.

### Labels

Labels are used exclusively for jumps, their refer to the actual program length.

Example:

```
ascii %0 'Hello, World !'
:a_label
hlt
```

Here, `a_label` value is 17.

## Command Line Interface

The command line (install it via `cargo install wavm` or by downloading a binary in [the release page](https://github.com/wafelack/wavm/releases)) has two major commands.

### Build

The build command compiles a `.wavm` source file to a `.wavc` bytecode file.

Usage: `wavm build <input_file> [-o [output file]]`.

This might raise errors in the following cases:

- Source file contains errors.
- Source file does not exist.

### Run

The run command runs a `.wavc` bytecode file inside the vm.

Usage: `wavm run <input_file>`.

This might raise errors in the following cases:

- Input file does not exist.
- Bytecode is invalid.
