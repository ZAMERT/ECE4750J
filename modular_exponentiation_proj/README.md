# Implemention of RSA

This program has implemented the classic modular and exponentiation algorithm as well as the faster square and multiplication described in homework 7. So this program has verified that the newly described algorithm compute the same answer with the classic one. Also, it shows that definitely method 2 is faster. 

## Features

- Implement the classic and newly described square and multiplication algorithm.
- Include the time needed for each algorithm respectively, and thus showing that the second method is faster. 

## Requirements

- Implemented in Rust.
- Need the cargo package.

## Dependencies

This project requires the following Rust crates:

- [`rug`] for arbitrary-precision arithmetic.

## Usage
```bash
cargo build --release
cargo run --release
```