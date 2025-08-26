# Implemention of extended Euclidean algorithm and Verification

This program has implemented the extended Euclidean algorithm, and has verified the result by comparing with the corresponding GMP function. It uses the rug crate, which can deal with integers with arbitrary precision. It also demonstrates the Bezout's identity, with two integers s and t, such that as + bt = gcd(a,b).

## Features

-Handle integers with arbitrary precision.
-Compare the built-in gcd function and the self implemented gcd with extended Euclidean algorithm.
-Verify the Bezout's identity that exist two integers s and t: as + bt = gcd(a,b).

## Requirements

-Implemented in Rust.
-Need the cargo package.

## Usage
```bash
cargo build --release
cargo run --release
```