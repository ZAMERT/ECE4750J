use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::io;

/// 快速幂模运算: base^exp mod modulus
fn mod_exp(base: &BigUint, exp: &BigUint, modulus: &BigUint) -> BigUint {
    let mut result = BigUint::one();
    let mut base = base.clone() % modulus;
    let mut exp = exp.clone();

    while !exp.is_zero() {
        if &exp & BigUint::one() == BigUint::one() {
            result = (&result * &base) % modulus;
        }
        base = (&base * &base) % modulus;
        exp >>= 1;
    }
    result
}

fn main() {
    println!("Enter base, exponent, and modulus (separated by space):");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<&str> = input.trim().split_whitespace().collect();

    if nums.len() != 3 {
        eprintln!("Please enter exactly three numbers!");
        return;
    }

    let base = BigUint::parse_bytes(nums[0].as_bytes(), 10).unwrap();
    let exp = BigUint::parse_bytes(nums[1].as_bytes(), 10).unwrap();
    let modulus = BigUint::parse_bytes(nums[2].as_bytes(), 10).unwrap();

    let result = mod_exp(&base, &exp, &modulus);

    println!("Result: {}", result);
}
