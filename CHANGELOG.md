# Changelog

## [0.2.0] - 2023-11-06

### Command-line arguments

```
Usage: death [OPTIONS]

Options:
  -n, --name <NAME>           Your name
  -b, --birthday <BIRTHDAY>   Your birthday
  -d, --death-reasons <FILE>  Custom death reasons file
  -h, --help                  Print help
  -V, --version               Print version
```

- Load your own death reasons with `--death-reasons <FILE>` argument. Each
death reason is on separate line.
- Added some colors to output of errors and enhanced general experience.

### API changes

This version introduces breaking API changes. You need to update your program
if you used `death` as a library. Also predictions will be different compared
to the previous version.
