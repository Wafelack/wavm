# Boolean

This is the boolean section opcodes documentation.

## EQ

- Byte identificator: `0x07`
- Description: Compares two register and puts the result in the 31th register.

- Operands table:

| Position |   Description   |
|----------|-----------------|
|    0     | The A register. |
|    1     | The B register. |

- Example code:

```text
eq %0 %1
```

- Errors:
    - InvalidRegister: Register A or B is not between 0 and 31.

## GEQ

- Byte identificator: `0x08`
- Description: Puts regA >= regB in the 31th register.

- Operands table:

| Position |   Description   |
|----------|-----------------|
|    0     | The A register. |
|    1     | The B register. |

- Example code:

```text
geq %0 %1
```

- Errors:
    - InvalidRegister: Register A or B is not between 0 and 31.

## GE

- Byte identificator: `0x09`
- Description: Puts regA > regB in the 31th register.

- Operands table:

| Position |   Description   |
|----------|-----------------|
|    0     | The A register. |
|    1     | The B register. |

- Example code:

```text
ge %0 %1
```

- Errors:
    - InvalidRegister: Register A or B is not between 0 and 31.

## LEQ

- Byte identificator: `0x0A`
- Description: Puts regA <= regB in the 31th register.

- Operands table:

| Position |   Description   |
|----------|-----------------|
|    0     | The A register. |
|    1     | The B register. |

- Example code:

```text
leq %0 %1
```

- Errors:
    - InvalidRegister: Register A or B is not between 0 and 31.

## LE

- Byte identificator: `0x0B`
- Description: Puts regA < regB in the 31th register.

- Operands table:

| Position |   Description   |
|----------|-----------------|
|    0     | The A register. |
|    1     | The B register. |

- Example code:

```text
le %0 %1
```

- Errors:
    - InvalidRegister: Register A or B is not between 0 and 31.

## NEQ

- Byte identificator: `0x0C`
- Description: Puts regA != regB in the 31th register.

- Operands table:

| Position |   Description   |
|----------|-----------------|
|    0     | The A register. |
|    1     | The B register. |

- Example code:

```text
neq %0 %1
```

- Errors:
    - InvalidRegister: Register A or B is not between 0 and 31.

## NOT

- Byte identificator: `0x0D`
- Description: Puts !reg in the 31th register.

- Operands table:

| Position |   Description   |
|----------|-----------------|
|    0     | The register. |

- Example code:

```text
not %0
```

- Errors:
    - InvalidRegister: Register is not between 0 and 31.

## OR

- Byte identificator: `0x0E`
- Description: Puts regA | regB in the 31th register.

- Operands table:

| Position |   Description   |
|----------|-----------------|
|    0     | The A register. |
|    1     | The B register. |

- Example code:

```text
or %0 %1
```

- Errors:
    - InvalidRegister: Register A or B is not between 0 and 31.

## AND

- Byte identificator: `0x0F`
- Description: Puts regA & regB in the 31th register.

- Operands table:

| Position |   Description   |
|----------|-----------------|
|    0     | The A register. |
|    1     | The B register. |

- Example code:

```text
and %0 %1
```

- Errors:
    - InvalidRegister: Register A or B is not between 0 and 31.

## XOR

- Byte identificator: `0x10`
- Description: Puts regA ^ regB in the 31th register.

- Operands table:

| Position |   Description   |
|----------|-----------------|
|    0     | The A register. |
|    1     | The B register. |

- Example code:

```text
xor %0 %1
```

- Errors:
    - InvalidRegister: Register A or B is not between 0 and 31.


