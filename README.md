### exit-with

[![crates.io][crates-badge]][crates-url]
![build][build-badge]
[![docs][docs-badge]][docs-url]
![coverage][cov-badge]
![license][license-badge]
![human][human-badge]

[crates-badge]: https://img.shields.io/crates/v/exit-with.svg
[crates-url]: https://crates.io/crates/exit-with
[build-badge]: https://github.com/EngosSoftware/exit-with/actions/workflows/build.yml/badge.svg
[docs-badge]: https://docs.rs/exit-with/badge.svg
[docs-url]: https://docs.rs/exit-with
[cov-badge]: https://img.shields.io/badge/coverage-100%25-21b577.svg
[license-badge]: https://img.shields.io/badge/license-MIT_OR_Apache--2.0-9370DB.svg
[human-badge]: https://img.shields.io/badge/made_by-HUMAN-F5DEB3.svg

# Exit with code, stdout & stderr

## Overview

**exit-with** is a minimalistic command-line utility that terminates
immediately after starting, while controlling the exit code,
standard output (stdout) and standard error (stderr) messages,
all in a single invocation.

## Usage

```text
exit-with [arg]... 
```

All arguments are optional. Each argument can be either a literal value, or a file name.
If file name is provided, then the contents of the file is used as the argument value.

> [!NOTE]
> Because of the specific usage of the **exit-with** utility,
> the only outputs from this application are the exit code and stdout/stderr messages.
> There are no additional options such as displaying version information,
> displaying help, or handling invalid arguments with error messages.

## How it works

The behavior of this application is defined by the following decision table:

|        |       |    1    |    2    |    3    |
|--------|-------|:-------:|:-------:|:-------:|
| i32    | `in`  |  true   |  true   |  false  |
| arg1   | `in`  |   = 0   |  != 0   |    -    |
| code   | `out` |    0    |  arg1   |    0    |
| stdout | `out` |  arg2   | arg3... |  arg1   |
| stderr | `out` | arg3... |  arg2   | arg2... |

`i32` and `arg1` are inputs. `code`, `stdout` and `stderr` are outputs.
`i32` is a function that returns `true` when `arg1` is a 32-bit signed integer.  

There are three rules defined in columns numbered from 1 to 3.

**RULE 1**

_when_

- the first argument `arg1` provided to `exit-with` **is** a 32-bit signed integer

_and_
 
- the first argument `arg1` **is equal** to zero
 
_then_ 

- the returned exit code will be **0**,
- the second argument `arg2` will be printed to **stdout**, 
- all arguments starting from`arg3...` will be printed to **stderr**. 

**RULE 2**

_when_

- the first argument `arg1` provided to `exit-with` **is** a 32-bit signed integer

_and_
 
- the first argument `arg1` **is not equal** to zero
 
_then_ 

- the returned exit code will be the value of `arg1`,
- all arguments starting from`arg3...` will be printed to **stdout** (sic!),
- the second argument `arg2` will be printed to **stderr** (sic!).

**RULE 3**
 
_when_

- the first argument `arg1` provided to `exit-with` **is not** a 32-bit signed integer
 
_then_ 

- the returned exit code will be **0**,
- the first argument `arg1` will be printed to **stdout**,
- all arguments starting from`arg2...` will be printed to **stderr**.

> [!TIP]
> The destination where the messages are printed out depends on the value of **code**.

## Examples

| arguments       | exit code | stdout | stderr |
|:----------------|:---------:|:-------|:-------|
|                 |     0     |        |        |
| `0`             |     0     |        |        |
| `hello`         |     0     | hello  |        |
| `0 hello`       |     0     | hello  |        |
| `0 hello world` |     0     | hello  | world  |
| `0 "" world`    |     0     |        | world  |
| `1`             |     1     |        |        |
| `1 hello`       |     1     |        | hello  |
| `1 hello world` |     1     | world  | hello  |
| `1 "" world`    |     1     | world  |        |

## License

Licensed under either of

- [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
  ([LICENSE-APACHE](LICENSE-APACHE) and [NOTICE](NOTICE))
- [MIT license](https://opensource.org/licenses/mit)
  ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Contribution

Any contributions intentionally submitted for inclusion in the work by you,
shall be dual licensed as above, without any additional terms or conditions.
