use rug::Integer;
use rug::Complete;
use std::time::Instant;

fn mod_exp(m: &Integer, d: &Integer, n: &Integer) -> Integer {
    let mut pow = Integer::from(1);
    let k = d.clone().significant_bits();
    for i in (0..k).rev() {
        pow = pow.square() % n;
        if d.get_bit(i) {
            pow = (pow * m) % n;
        }
    }

    pow
}

fn faster_mod_exp(product: &Integer, alpha: &Integer, beta: &Integer, a: &Integer, b: &Integer, n: &Integer) -> Integer {
    let mut pow = Integer::from(1);
    let l1 = a.clone().significant_bits();
    let l2 = b.clone().significant_bits();
    let l3 = std::cmp::max(l1, l2);

    for i in (0..l3).rev() {
        pow = pow.square() % n;
        if a.get_bit(i) && b.get_bit(i) {
            pow = (pow * product) % n;
        } else if a.get_bit(i) {
            pow = (pow * alpha) % n;
        } else if b.get_bit(i) {
            pow = (pow * beta) % n;
        }
    }

    pow
}

fn main() {
    let alpha = Integer::from(139152348);
    let beta = Integer::from(123134545);
    let a = Integer::from(8573344);
    let b = Integer::from(34751359);
    let n = Integer::from(34827565);

    let product = ((&alpha * &beta)).complete() % &n.clone();

    let start1 = Instant::now();
    let result1_a = mod_exp(&alpha, &a, &n);
    let result1_b = mod_exp(&beta, &b, &n);
    let result1 = (&result1_a * &result1_b).complete() % &n.clone();
    let duration1 = start1.elapsed();

    let start2 = Instant::now();
    let result2 = faster_mod_exp(&product, &alpha, &beta, &a, &b, &n);
    let duration2 = start2.elapsed();

    assert_eq!(result1, result2);
    println!("Result: {}", result2);
    println!("mod_exp time:    {:?}", duration1);
    println!("faster_mod_exp time: {:?}", duration2);
}