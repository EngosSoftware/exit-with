# Exit with code, stdio & stderr

## Overview

Minimalistic command-line utility that terminates immediately after starting,
while controlling the exit code, standard output (stdout) message
and standard error (stderr) message in a single invocation.

## Usage

```text
exit-with [code] [msg-1] [msg-2] 
```

## Examples

| code | stdout | stderr | command                 |
|:----:|:------:|:------:|-------------------------|
|  0   |   -    |   -    | `exit-with`             |
|  0   |   -    |   -    | `exit-with 0`           |
|  0   | hello  |   -    | `exit-with hello`       |
|  0   | hello  | world  | `exit-with hello world` |
