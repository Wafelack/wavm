# Arithmetic

This is the arithmetic section opcodes documentation.

## ADD

- Byte identificator: `0x02`
- Description: Adds a register to another register.

- Operands table:

| Position |   Description   |
|----------|-----------------|
|    0     | The A register. |
|    1     | The B register. |

- Example code:

```text
add %0 %1
```

- Errors:
    - InvalidRegister: Register A or B is not between 0 and 31.

## SUB

- Byte identificator: `0x03`
- Description: Substracts a register from another register.

- Operands table:

| Position |   Description   |
|----------|-----------------|
|    0     | The A register. |
|    1     | The B register. |

- Example code:

```text
sub %0 %1
```

- Errors:
    - InvalidRegister: Register A or B is not between 0 and 31.

## MUL

- Byte identificator: `0x04`
- Description: Multiplies a register by another register.

- Operands table:

| Position |   Description   |
|----------|-----------------|
|    0     | The A register. |
|    1     | The B register. |

- Example code:

```text
mul %0 %1
```

- Errors:
    - InvalidRegister: Register A or B is not between 0 and 31.

## DIV

- Byte identificator: `0x05`
- Description: Divides a register by another register.

- Operands table:

| Position |   Description   |
|----------|-----------------|
|    0     | The A register. |
|    1     | The B register. |

- Example code:

```text
div %0 %1
```

- Errors:
    - InvalidRegister: Register A or B is not between 0 and 31.

## MOD

- Byte identificator: `0x05`
- Description: Set a register to the reminder of its euclidian division by another register.

- Operands table:

| Position |   Description   |
|----------|-----------------|
|    0     | The A register. |
|    1     | The B register. |

- Example code:

```text
mod %0 %1
```

- Errors:
    - InvalidRegister: Register A or B is not between 0 and 31.
