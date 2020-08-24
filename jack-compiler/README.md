# jack-compiler

An implementation of the Jack Compiler written in Rust.  *(The Elements of Computing Systems - Project 10/11)*


## Components

- jack-analyzer:  .jack -> .xml
- jack-compiler:  .jack -> .vm  (Todo)


## Usage

```
# Single .jack file
jack-compiler <file.jack>

# Multiple .jack files
jack-compiler <dir>
```

## Installation

Requires the [Rust Toolchain](https://www.rust-lang.org/tools/install).


jack-analyzer:
```
cargo install --git https://github.com/hamish-miller/nand2tetris-toolchain jack-analyzer
```
