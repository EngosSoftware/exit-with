# Exit with code, stdout & stderr

## Overview

Minimalistic command-line utility that terminates immediately after starting,
while controlling the exit code, standard output (stdout)
and standard error (stderr) messages, all in a single invocation.

## Usage

```text
exit-with [arg]... 
```

All arguments are optional. Each argument can be either a literal value, or a file name.
If a file name is provided, the file's contents is used as the argument value.

> [!NOTE]
> Because of the specific usage of this utility application,
> the only outputs are: exit code, stdout message and stderr message.
> There are no additional options such as displaying version information,
> displaying help text, or handling invalid arguments with error messages.

## How it works

The behavior of this application is defined by the following decision table:

|           |       |    1    |    2    |    3    |
|-----------|-------|:-------:|:-------:|:-------:|
| i32(arg1) | `in`  |  true   |  true   |  false  |
| arg1      | `in`  |   = 0   |  != 0   |    -    |
| code      | `out` |    0    |  arg1   |    0    |
| stdout    | `out` |  arg2   | arg3... |  arg1   |
| stderr    | `out` | arg3... |  arg2   | arg2... |

`i32(arg1)`, `arg1` are inputs, `code`, `stdout`, `stderr` are outputs.
`i32(x)` is a function that returns `true` when `x` is a 32-bit signed integer.  

There are three rules defined in columns numbered 1 to 3.

> **RULE 1**

WHEN

- the first argument `arg1` provided to `exit-with` is a 32-bit signed integer

AND
 
- the first argument `arg1` is **equal** to zero
 
THEN 

- the returned exit code will be `0`,
- the second argument `arg2` will be printed to _stdout_, 
- all arguments starting from`arg3...` will be printed to _stderr_. 

> **RULE 2**
 
WHEN

- the first argument `arg1` provided to `exit-with` is a 32-bit signed integer

AND
 
- the first argument `arg1` is **not equal** to zero
 
THEN 

- the returned exit code will be the value of `arg1`,
- all arguments starting from`arg3...` will be printed to _stdout_ (sic!),
- the second argument `arg2` will be printed to _stderr_ (sic!).

> **RULE 3**
 
WHEN

- the first argument `arg1` provided to `exit-with` is **not** a 32-bit signed integer
 
THEN 

- the returned exit code will be `0`,
- the first argument `arg1` will be printed to _stdout_,
- all arguments starting from`arg2...` will be printed to _stderr_.

> [!TIP]
> The destination where messages are printed out depends on the value of **code**.

## Examples

| No. | arguments     | code | stdout | stderr |
|----:|:--------------|:----:|:-------|:-------|
|   1 |               |  0   |        |        |
|   2 | 0             |  0   |        |        |
|   3 | hello         |  0   | hello  |        |
|   4 | 0 hello       |  0   | hello  |        |
|   5 | 0 hello world |  0   | hello  | world  |
|   6 | 0 "" world    |  0   |        | world  |
|   7 | 1             |  1   |        |        |
|   8 | 1 hello       |  1   |        | hello  |
|   9 | 1 hello world |  1   | world  | hello  |
|  10 | 1 "" world    |  1   | world  |        |
