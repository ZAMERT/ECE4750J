use rug::Integer;
use std::io;

fn fx(x: &Integer, n: &Integer) -> Integer {
    (x * x + Integer::from(1)) % n
}

fn pollard_rho(n: &Integer) -> Integer {
    let mut a = Integer::from(2);
    let mut b = Integer::from(2);
    if n.is_even() {
        return Integer::from(2);
    }

    let mut d = Integer::from(1);
    while d == Integer::from(1) {
        a = fx(&a, n);
        b = fx(&fx(&b, n), n);
        d = Integer::from(&a - &b).abs().gcd(n);
    }

    if &d == n {
        return Integer::from(-1);
    }

    d
}

fn main() {
    println!("Enter a number to factorize: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let n: Integer = input.trim().parse().unwrap();

    let d = pollard_rho(&n);

    if d == Integer::from(-1) {
        println!("Failed to find a non-trivial factor of {}", n);
        return;
    }

    println!("A non-trivial factor of {} is: {}", n, d);
}
