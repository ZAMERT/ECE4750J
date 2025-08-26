# Implemention of RSA

This program has implemented the RSA cryptosystem. To be specific, this program has completed the three main functions of RSA: generate, encrypt and decrypt. It uses the rug crate, which can deal with integers with arbitrary precision. It can also generate long enough n so as to match different required security level. 

## Features

- Include the generate, encrypt and decrypt functions of RSA.
- Handle integers with arbitrary precision.
- Match different required security level.

## Requirements

- Implemented in Rust.
- Need the cargo package.

## Dependencies

This project requires the following Rust crates:

- [`rug`] for arbitrary-precision arithmetic.
- [`rand`] for random number generation.

## Usage
```bash
cargo build --release
cargo run --release
```