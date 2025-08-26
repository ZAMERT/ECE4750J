use rug::Integer;
use rug::rand::RandState;
use rug::integer::IsPrime;

fn main() {
    let security_level = 192;
    let n_bits = security_level_pair(security_level);
    let (p,q) = generator(n_bits);
    let (public_key, private_key) = key_generate(p, q);
    let plain = Integer::from(135792468);
    let cipher = encrypt(&public_key, &plain);
    let cipher_length = cipher.significant_bits();
    let de_plain = decrypt(&private_key, &cipher);
    println!("Plaintext: {}", plain);
    println!("Ciphertext: {}", cipher);
    println!("Ciphertext Length: {} bits", cipher_length);
    println!("Decrypted Plaintext: {}", de_plain);
}

struct RSA_PublicKey {
    n: Integer,
    e: Integer,
}

struct RSA_PrivateKey {
    n: Integer,
    d: Integer
}


fn security_level_pair(security_level: u32) -> u32 {
    match security_level {
        80 => 1024,
        112 => 2048,
        128 => 3072,
        192 => 7680,
        256 => 15360,
        _ => panic!("Invalid security level: {}", security_level),
    }
}

fn generate_prime(p_bits: u32) -> Integer {
    let seed = rand::random::<u64>();
    let mut rand = RandState::new();
    rand.seed(&seed.into());
    loop {
        let prime_candidate = Integer::from(Integer::random_bits(p_bits, &mut rand));
        if prime_candidate.is_probably_prime(20) != IsPrime::No {
            return prime_candidate.into();
        }
    }
}

fn generator(n_bits: u32) -> (Integer, Integer) {
    let p_bits = n_bits / 2;
    let q_bits = n_bits - p_bits;

    let p = generate_prime(p_bits);
    let q = loop {
        let temp = generate_prime(q_bits);
        if temp != p {
            break temp;
        }
    };

    (p, q)
}

fn key_generate(p: Integer, q: Integer) -> (RSA_PublicKey, RSA_PrivateKey) {
    let n = Integer::from(&p * &q);
    let phi_n = Integer::from(&p - 1u32) * Integer::from(&q - 1u32);

    let e = Integer::from(65537);
    let d = e.clone().invert(&phi_n).expect("Failed to find inverse of e.");

    let public_key = RSA_PublicKey { n: n.clone(), e };
    let private_key = RSA_PrivateKey { n: n.into(), d };

    (public_key, private_key)
}

fn encrypt(public_key: &RSA_PublicKey, plain: &Integer) -> Integer {
    let mut cipher = plain.clone().pow_mod(&public_key.e, &public_key.n).expect("Failed to encrypt the message.");
    cipher
}

fn decrypt(private_key: &RSA_PrivateKey, cipher: &Integer) -> Integer {
    let mut plain = cipher.clone().pow_mod(&private_key.d, &private_key.n).expect("Failed to decrypt the message.");
    plain
}