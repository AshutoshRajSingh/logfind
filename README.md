# Logfind

An easy to use automatic log searching utility written in rust, inspired by Zed Shaw's textbook: Learn C the hard way, except my blasphemous self used rust, oh and by the way did I tell you it was written in rust?

## Usage:  
- Clone the repository
- Build logfind
- Use logfind

## Example

```shell
logfind nginx ubuntu --all --minimal
```

Would search log files for lines that contain both "nginx" and "ubuntu", and would display the output formatted in a minimal format.

```shell
logfind nginx ubuntu -any
```

Would search log files for lines containing any one of "nginx" or "ubuntu" or both, and display the output formatted in a minimal format.

## Features
1. Very easy to use, just two optional options to specify.
2. Very fast because it loads the entire files into memory, I mean who has log files that are large enough to not fit in memory, right?