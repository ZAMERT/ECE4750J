# Implemention of Advanced Encryption Standard(AES)

This program has implemented the algorithms of AES, which includes the procedures of AddRoundKey, SubBytes, ShiftRows and MixColumns respectively. At the same time, this program also has tried the generation of the S-Box, and has proved to be successful by comparing to the S-Box table. However, due to the limited time, this program hasn't tested the whole complete implementation of the AES, in other words, this program cannot achieve the combined, ten rounds AES encryption yet. But this is an attempt to implement the AES in Rust. 

## Features

-Implement the four key functions in AES encryption respectively. 
-Generate the S-Box.
-Use logical operators to complete the implementtaion of operations. 

## Requirements

-Implemented in Rust.
-Need the cargo package.

## Usage
```bash
cargo build --release
cargo run --release
```