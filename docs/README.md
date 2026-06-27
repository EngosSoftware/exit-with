# Exit with code, stdio & stderr

## Overview

Minimalistic command-line utility that terminates immediately after starting,
while controlling the exit code, standard output (stdout) message
and standard error (stderr) message in a single invocation.

## Usage

```text
exit-with [code] [msg-1] [msg-2] 
```

- **code** - optional exit code, default is 0 (zero).
- **msg-1** - optional message printed to:
    - `stdout` when **code** is zero,
    - `stderr` when **code** is nonzero.
- **msg-2** - optional message printed to:
    - `stderr` when **code** is zero,
    - `stdout` when **code** is nonzero.

> [!IMPORTANT]
> Please note, that the destination where **msg-1** and **msg-2**
> are printed depends on the value of **code**.

> [!TIP]
> Because of the specific usage of this application, the only outputs
> are an exit code, a stdout message, and a stderr message.
> There are no additional options such as displaying version information,
> displaying help text, or handling invalid arguments with error messages.

## Examples

| code | stdout | stderr | command                 |
|:----:|:------:|:------:|-------------------------|
|  0   |   -    |   -    | exit-with               |
|  0   |   -    |   -    | exit-with 0             |
|  0   | hello  |   -    | exit-with 0 hello       |
|  0   | hello  | world  | exit-with 0 hello world |
|  0   |   -    | world  | exit-with 0 "" world    |
|  1   |   -    |   -    | exit-with 1             |
|  1   |   -    | hello  | exit-with 1 hello       |
|  1   | world  | hello  | exit-with 1 hello world |
|  1   | world  |   -    | exit-with 1 "" world    |
