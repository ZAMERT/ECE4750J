use rug::Integer;
use rug::rand::RandState;
use rug::Complete;

fn extended_euclidean(a: &Integer, b: &Integer) -> (Integer, Integer, Integer) {
    let mut r0 = b.clone();
    let mut r1 = a.clone();
    let mut s0 = Integer::from(0);
    let mut s1 = Integer::from(1);
    let mut t0 = Integer::from(1);
    let mut t1 = Integer::from(0);

    while r0 != 0{
        let q = (&r1 / &r0).complete();
        let temp1 = r0.clone();
        r0 = (&r1 - &q * &r0).into();
        r1 = temp1;
        let temp2 = s0.clone();
        s0 = (&s1 - &q * &s0).into();
        s1 = temp2;
        let temp3 = t0.clone();
        t0 = (&t1 - &q * &t0).into();
        t1 = temp3;
    }

    (r1, s1, t1)
}

fn main() {
    let seed = rand::random::<u64>();

    let mut rand = RandState::new();
    rand.seed(&seed.into());
    let a = Integer::from(Integer::random_bits(4096, &mut rand));
    let b = Integer::from(Integer::random_bits(4096, &mut rand));
    let gcd1 = a.clone().gcd(&b);
    let(gcd2, s, t) = extended_euclidean(&a, &b);

    println!("a = {}", a);
    println!("b = {}", b);
    println!("gcd = {}", gcd1);
    println!("gcd_SelfImplemented = {}", gcd2);
    println!("s * a + t * b = {}", (&s * &a).complete() + (&t * &b).complete());
}
