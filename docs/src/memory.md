# Memory

## RQM

- Byte identificator: `0x14`
- Description: Requests memory on the heap.

- Operands table:

| Position |               Description              |
|----------|----------------------------------------|
|    0     | The register where to put the pointer. |
|    1     |     The amount of bytes to request.    |

- Example code:

```text
rqm %0 0x456
```

- Errors:
  - The register is not a valid register.
  - The requested amount is too large (unless you are running that VM on a 70's machine, it should be ok.)

## ASCII

- Byte identificator: `0x15`
- Description: Places a string on the heap.

- Operands table:

| Position |                      Description                     |
|----------|------------------------------------------------------|
|    0     |        The register where to put the pointer.        |
|   1..n   | The string bytes (has to end with a null terminator).|

- Example code:

```text
ascii %0 'Hello, World !'
```

> NOTE: On the high level assembly, no need of specifying the null terminator, it is automatically added at compile time.

- Errors:
  - The register is not a valid register.
  - The requested amount is too large.

## SETB

- Byte identificator: `0x16`
- Description: Sets a byte to a value on the heap.

- Operands table:

| Position |                 Description                |
|----------|--------------------------------------------|
|    0     | The register where the pointer is loacted. |
|    1     |                The new byte.               |

- Example code

```text
rqm %0 2
setb %0 0x11
```
- Errors:
  - The register is not a valid register.
  - SegmentationFault: The memory at the pointer is not yet initialized.

## GETB

- Byte identificator: `0x17`
- Description: Gets a byte on the heap into a register.

- Operands table:

| Position |               Description              |
|----------|----------------------------------------|
|    0     |        The destination register.       |
|    1     |   The register containing the pointer  |

- Example code:

```text
rqm %0 2
setb %0 0x11
getb %1 %0
```

- Errors:
  - The destination/pointer register is not a valid register.
  - SegmentationFault: The memory at the pointer is not yet initialized.

## MMOV

- Byte identificator: `0x18`
- Description: Moves a piece of memory to another place.

- Operands table:

| Position |               Description              |
|----------|----------------------------------------|
|    0     |         The destination pointer.       |
|    1     |          The source pointer.           |
|    2     |       The amount of bytes to move.     |

- Example code:

```text
ascii %0 'Hello, World !'
rqm %1 0xF
mmov %0 %1 0xF
```

- Errors:
  - The destination/source register is not a valid register.
  - SegmentationFault: The destination/source pointer is not yet initialized.

## MSET

- Byte identificator: `0x19`
- Description: Sets a piece of memory to a specific byte.

- Operands table:

| Position |               Description              |
|----------|----------------------------------------|
|    0     |          The pointer's register        |
|    1     |            The byte to set.            |
|    2     |       The amount of bytes to set.      |

- Example code:

```text
rqm %0 0xF
mset %0 0xFF 0xF
```

- Errors:
  - The pointer register is not a valid register.
  - SegmentationFault: The bytes to set aren't initialized.

## FREE

- Byte identificator: `0x1A`
- Description: Frees a block of memory.

- Operands table:

| Position |               Description              |
|----------|----------------------------------------|
|    0     |           The register pointer.        |
|    1     |       The amount of bytes to free.     |

- Example code:

```text
ascii %0 'Hello, World !'
free %0 0xF
```

- Errors:
  - The pointer register is not a valid register.
  - SegmentationFault: The bytes to free aren't set.

