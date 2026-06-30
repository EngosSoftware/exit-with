### exit-with

[![crates.io][crates-badge]][crates-url]
[![coverage][cov-badge]][cov-url]
![build-linux-gnu][build-badge-linux-gnu]
![build-linux-musl][build-badge-linux-musl]
![build-macos][build-badge-macos]
![build-macos-aarch64][build-badge-macos-aarch64]
![build-windows][build-badge-windows]
[![mit-license][mit-badge]][mit-license-url]
[![apache-license][apache-badge]][apache-license-url]
[![cc][cc-badge]][cc-url]
[![mbh][mbh-badge]][mbh-url]
[![es][es-badge]][es-url]

[crates-badge]: https://img.shields.io/crates/v/exit-with.svg
[crates-url]: https://crates.io/crates/exit-with
[cov-badge]: https://img.shields.io/badge/coverage-100%25-21b577.svg
[cov-url]: https://crates.io/crates/coverio
[build-badge-linux-gnu]: https://github.com/EngosSoftware/exit-with/actions/workflows/build-linux-gnu.yml/badge.svg
[build-badge-linux-musl]: https://github.com/EngosSoftware/exit-with/actions/workflows/build-linux-musl.yml/badge.svg
[build-badge-macos]: https://github.com/EngosSoftware/exit-with/actions/workflows/build-macos.yml/badge.svg
[build-badge-macos-aarch64]: https://github.com/EngosSoftware/exit-with/actions/workflows/build-macos-aarch64.yml/badge.svg
[build-badge-windows]: https://github.com/EngosSoftware/exit-with/actions/workflows/build-windows.yml/badge.svg
[mit-badge]: https://img.shields.io/badge/License-MIT-9370DB.svg
[mit-url]: https://opensource.org/licenses/MIT
[mit-license-url]: https://github.com/EngosSoftware/exit-with/blob/main/LICENSE-MIT
[apache-badge]: https://img.shields.io/badge/License-Apache%202.0-9370DB.svg
[apache-url]: https://www.apache.org/licenses/LICENSE-2.0
[apache-license-url]: https://github.com/EngosSoftware/exit-with/blob/main/LICENSE
[apache-notice-url]: https://github.com/EngosSoftware/exit-with/blob/main/NOTICE
[cc-badge]: https://img.shields.io/badge/Contributor%20Covenant-2.1-9370DB.svg
[cc-url]: https://github.com/EngosSoftware/exit-with/blob/main/CODE_OF_CONDUCT.md
[mbh-badge]: https://img.shields.io/badge/Made_by-HUMAN-F5DEB3.svg
[mbh-url]: https://github.com/DariuszDepta
[es-badge]: https://img.shields.io/badge/at-Engos_Software-ADD8E6.svg
[es-url]: https://engos.de
[repository-url]: https://github.com/EngosSoftware/exit-with

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

## License

Licensed under either of

- [MIT license][mit-url] (see [LICENSE-MIT][mit-license-url]) or
- [Apache License, Version 2.0][apache-url] (see [LICENSE][apache-license-url] and [NOTICE][apache-notice-url])

at your option.

## Contribution

Any contributions to [exit-with][repository-url] are greatly appreciated.
All contributions intentionally submitted for inclusion in the work by you,
shall be dual licensed as above, without any additional terms or conditions.
