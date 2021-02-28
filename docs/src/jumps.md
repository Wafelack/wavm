# Jumps

## JMP

- Byte identificator: `0x11`
- Description: Jumps to an instruction.

- Operands table:

| Position |            Description            |
|----------|-----------------------------------|
|    0     | The instruction index to jump to. |

> NOTE: The instruction can also be [a label](./intro.md#labels).

- Example code:

```text
jmp 0x01
```

- Errors:
  - Instruction index is out of bounds.
  - Instruction index is not a valid unsigned 16 bits integer.

## JMPEQ

- Byte identificator: `0x12`
- Description: Jumps to an instruction if the `$EQ` flag is set to 1.

| Position |            Description            |
|----------|-----------------------------------|
|    0     | The instruction index to jump to. |

> NOTE: The instruction can also be [a label](./intro.md#labels).

- Example code:

```text
jmpeq 0x01
```

- Errors:
  - Instruction index is out of bounds.
  - Instruction index is not a valid unsigned 16 bits integer.

## RJMP

- Byte identificator: `0x13`
- Description: Jumps to an instruction with a relative index.

| Position |                          Description                           |
|----------|----------------------------------------------------------------|
|    0     | The instruction index to jump to from the current instruction. |

> NOTE: The instruction can also be [a label](./intro.md#labels).

- Example code:

```text
rjmp 0x01
```

- Errors:
  - Instruction pointer + Instruction index is out of bounds.
  - Instruction index is not a valid unsigned 16 bits integer.

