# Miscellanous

This is the `Miscellanous` section opcodes documentation.

## LOAD

- Byte identificator: `0x00`
- Description: Loads a value into a register.

- Operands table:

| Position |  Description  |
|----------|---------------|
|    0     | The register. |
|    1     |   The value.  |

- Example code:

```text
load %0 0xFF
```

- Errors:
    - InvalidRegister: Register is not between 0 and 31.
    - OutOfBounds: Value does not fit on unsigned 16 bits.

## MOVE

- Byte identificator: `0x01`
- Description: Copies a value from one register to another.

- Operands table:

| Position |   Description   |
|----------|-----------------|
|    0     | The A register. |
|    1     | The B register. |

- Example code:

```text
move %0 %1
```

- Errors:
    - InvalidRegister: Register A or B is not between 0 and 31.

## HLT

- Byte identificator: `0xCC`
- Description: Sets the program state to false.

- Example code:

```text
hlt
```

## DRG

- Byte identificator: `0x99`
- Description: Displays a register row value.

- Operands table:

| Position |  Description  |
|----------|---------------|
|    0     | The register. |

- Example code:

```text
drg %0
```

- Errors:
    - InvalidRegister: Register is not between 0 and 31.

## DSP

- Byte identificator: `0xAA`
- Description: Displays a register pointer value.

- Operands table:

| Position |  Description  |
|----------|---------------|
|    0     | The register. |

- Example code:

```text
dsp %0
```

- Errors:
    - InvalidRegister: Register is not between 0 and 31.
