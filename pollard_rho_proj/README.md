# Implemention of Pollard-Rho Algorithm

This program has implemented the Pollard-Rho Algorithm, which is a easier way to factorize some quite big n. It can be used when small numbers have been eliminated as possible factors, and is faster than trial factorization. 

## Features

- Faster than trial factorization.

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