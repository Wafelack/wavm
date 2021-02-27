# Introduction

WAVM is a 64 bits register based virtual machine with a few opcodes, written in Rust.

## Overview

This virtual machine has 32 registers defined by the following table.

| Number |     Usage      |
|--------|----------------|
|  0..30 |  Multi purpose |
|   31   |  Equality flag |

---

There are 31 opcodes available, defined by the following table.

| Name | Bytecode Value | Section |
|------|----------------|---------|
| LOAD |     0x00       | Miscellanous |
| MOVE |     0x01       | Miscellanous |
| HLT  |     0xCC       | Miscellanous |
| DRG  |     0x99       | Miscellanous |
| DSP  |     0xAA       | Miscellanous |
| ADD  |     0x02       |  Arithmetic  |
| SUB  |     0x03       |  Arithmetic  |
| MUL  |     0x04       |  Arithmetic  |
| DIV  |     0x05       |  Arithmetic  |
| MOD  |     0x06       |  Arithmetic  |
|  EQ  |     0x07       |   Boolean    |
| GEQ  |     0x08       |   Boolean    |
|  GE  |     0x09       |   Boolean    |
| LEQ  |     0x0A       |   Boolean    |
|  LE  |     0x0B       |   Boolean    |
| NEQ  |     0x0C       |   Boolean    |
|  NOT |     0x0D       |   Boolean    |
|  OR  |     0x0E       |   Boolean    |
| AND  |     0x0F       |   Boolean    |
| XOR  |     0x10       |   Boolean    |
|  JMP |     0x11       |    Jumps     |
|JMPEQ |     0x12       |    Jumps     |
| RJMP |     0x13       |    Jumps     |
| RQM  |     0x14       |    Memory    |
|ASCII |     0x15       |    Memory    |
| SETB |     0x16       |    Memory    |
| GETB |     0x17       |    Memory    |
| MMOV |     0x18       |    Memory    |
| MSET |     0x19       |    Memory    |
| FREE |     0x1A       |    Memory    |


## Command Line Interface

