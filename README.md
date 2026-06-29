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

## License

Licensed under either of

- [MIT license][mit-url] (see [LICENSE-MIT][mit-license-url]) or
- [Apache License, Version 2.0][apache-url] (see [LICENSE][apache-license-url] and [NOTICE][apache-notice-url])

at your option.

## Contribution

Any contributions to [exit-with][repository-url] are greatly appreciated.
All contributions intentionally submitted for inclusion in the work by you,
shall be dual licensed as above, without any additional terms or conditions.
