# grep

An implementation of `grep` in Rust with colored match highlighting.

## Usage
```
Usage: grep [OPTIONS] --pattern <PATTERN> --path <PATH>

Options:
    -p, --pattern <PATTERN>                The pattern to search for
    -f, --path <PATH>                      The file/dir to search in
    -r, --recursive                        Search recursively through directories
    -i, --ignore-case                      Ignore case when matching
    -B, --before-context <BEFORE_CONTEXT>  Print NUM lines of leading context
    -A, --after-context <AFTER_CONTEXT>    Print NUM lines of trailing context
    -C, --context <CONTEXT>                Print NUM lines of context
    -n, --line-numbers                     Show line numbers
        --no-color                         Disable colored output
    -h, --help                             Print help
    -V, --version                          Print version
```

## Build
```
cargo build
```

## License
MIT
