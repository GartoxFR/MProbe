# Introduction

The point of this work is to build tools and a set of tests to measure
memory consumption during the compilation and execution of programs 
implemented in different language

# Tools

## Dependencies

- Rust toolchain (see the [Rust book](https://doc.rust-lang.org/book/ch01-01-installation.html))
- Python3 and pip3
- matplotlib (`pip3 install matplotlib`)

## Building

```sh
make tools
```

# Tests

All the program we mesure on are located in the `tests` directory. They are here sorted by language

To run a test located there you can use :
```sh
make O=<output-dir> <language-folder>/<test-folder>/<step>
```

| <step>  | Description                                 |
|---------|---------------------------------------------|
| `build` | Mesure only the memory used at compile time |
| `exec`  | Mesure only the memory used at run time     |
| `all`   | Mesure both at compile time and run time    |
