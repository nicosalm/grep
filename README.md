# grep

A simple grep implementation in Rust that searches files for patterns.

## Quick Start
```bash
# search a file
grep -p "pattern" -f file.txt

# search with context
grep -p "pattern" -f file.txt -C 2

# search directory recursively
grep -p "pattern" -f ./src -r
```

## Options
```
-p, --pattern        Pattern to search for
-f, --path           File/directory to search
-r, --recursive      Search directories recursively
-i, --ignore-case    Case insensitive search
-B NUM               Show NUM lines before match
-A NUM               Show NUM lines after match
-C NUM               Show NUM lines before and after match
```

## Build
```bash
cargo build
```

## License
MIT
