use rand::Rng;

fn main() {
    let sbox = sbox_generated();
    let mut round_key = [[0u8; 4]; 44];

    let mut plaintext = [[0u8; 4]; 4];
    let mut rng = rand::thread_rng();
    for i in 0..4 {
        for j in 0..4 {
            plaintext[i][j] = rng.r#gen();
            print!("{:02x} ", plaintext[i][j]);
        }
    }

    let mut state = [[0u8; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            state[i][j] = plaintext[i][j];
        }
    }

    for i in 0..4 {
        for j in 0..4 {
            round_key[i][j] = plaintext[j][i];
        }
    }

    sub_bytes(&mut state, &sbox);
    shift_rows(&mut state);
    mix_columns(&mut state);
}

// Implementation of Galois Field multiplication.
fn gf_multiple(mut a:u8, mut b:u8) -> u8 {
    let mut result = 0u8;
    for _i in 0..8 {
        // Seperate b into bits, consider it as a(x) + a(x)*x + a(x)*x^2 + ... + a(x)*x^7
        if (b & 1) != 0 {
            result ^= a; // gf_add is just xor here.
        }
        let bit7 = a & 0x80; // Check if the first bit is 0.
        a <<= 1;
        if (bit7) != 0 { // If the first bit is 0 stop and otherwise XOR with P(X). While P(X) = x^8 + x^4 + x^3 + x + 1.
            a ^= 0x1b;
        }
        b >>= 1;
    }
    result
}

// Implementation of Galois Field inverse.
fn gf_inverse(a: u8) -> u8 {
    if a == 0 {
        return 0;
    }
    for i in 1..=255 {
        if gf_multiple(a, i) == 1 { // May be optimized here.
            return i;
        }
    }
    panic!("No inverse found for {}", a);
}

fn sbox_generated() -> [[u8; 16]; 16] {
    let mut sbox = [[0u8; 16]; 16];
    for i in 0..16 {
        for j in 0..16 {
            let a = (i << 4) | j; // Combine the separate 4-bits num a and b into a byte.
            let mut a_inv = 0u8;

            // Calculate the multiplicative inverse.
            if a == 0 {
                a_inv = 0;
            }
            else {
                a_inv = gf_inverse(a);
            }

            // Calculate the fixed matrix multiplied by the inverse, and then xor with the constant 0x63. 
            let mut temp = 0u8;
            for k in 0..8 {
                let bit = ((a_inv >> k) & 1) ^ ((a_inv >> ((k + 4) % 8)) & 1) ^ ((a_inv >> ((k + 5) % 8)) & 1) ^ ((a_inv >> ((k + 6) % 8)) & 1) ^ ((a_inv >> ((k + 7) % 8)) & 1);
                temp |= bit << k;
            }
            temp ^= 0x63;

            sbox[i as usize][j as usize] = temp;
        }
    }
    sbox
}

// Implementation of SubBytes.
fn sub_bytes(state: &mut [[u8; 4]; 4], sbox: &[[u8; 16]; 16]) {
    for i in 0..4 {
        for j in 0..4 {
            let byte = state[i][j];
            let a = byte >> 4; // The first 4 bits.
            let b = byte & 0xF; // The last 4 bits.
            state[i][j] = sbox[a as usize][b as usize];
        }
    }
}

// Implementation of ShiftRows
fn shift_rows(state: &mut [[u8; 4]; 4]) {
    for i in 1..4 {
        let row = state[i]; // Get the i-th row.
        for j in 0..4 {
            state[i][j] = row[(i+j) % 4];
        }
    }
}

// Implementation of MixColumns
fn mix_columns(state: &mut [[u8; 4]; 4]) {
    for i in 0..4{
        let a0 = state[0][i];
        let a1 = state[1][i];
        let a2 = state[2][i];
        let a3 = state[3][i];

        state[0][i] = gf_multiple(0x02, a0) ^ gf_multiple(0x03, a1) ^ a2 ^ a3;
        state[1][i] = a0 ^ gf_multiple(0x02, a1) ^ gf_multiple(0x03, a2) ^ a3;
        state[2][i] = a0 ^ a1 ^ gf_multiple(0x02, a2) ^ gf_multiple(0x03, a3);
        state[3][i] = gf_multiple(0x03, a0) ^ a1 ^ a2 ^ gf_multiple(0x02, a3);
    }
}

fn cyclical_shift(k: [u8; 4]) -> [u8; 4] {
    let mut k_temp = k;
    let temp = k_temp[0];
    for i in 0..3 {
        k_temp[i] = k_temp[i + 1];
    }
    k_temp[3] = temp;
    k_temp
}

fn r_generated(n: usize) -> u8 {
    let mut r = 1u8;
    for _ in 0..n {
        r = gf_multiple(r, 0x02);
    }
    r
}

fn transformation_round_key(i: u8, k: [u8; 4], sbox: &[[u8; 16]; 16]) -> [u8; 4] {
    let mut next_k_i = [0u8; 4];
    let r_i = r_generated(((i-4)/4) as usize);
    next_k_i = cyclical_shift(k);
    for j in 0..4 {
        next_k_i[j] = sbox[(next_k_i[j] >> 4) as usize][(next_k_i[j] & 0x0F) as usize];
    }
    next_k_i[0] ^= r_i;
    next_k_i
}

fn xor_array(a: [u8;4], b: [u8; 4]) -> [u8; 4]{
    let mut result = [0u8; 4]; 
    for i in 0..4 {
        result[i] = a[i] ^ b[i];
    }
    result
}

fn round_key_generated(i: u8, k_i4: [u8; 4], k_i1: [u8; 4], sbox: &[[u8; 16]; 16]) -> [u8; 4] {
    let mut new_ki = [0u8; 4];
    if i % 4 == 0 {
        new_ki = xor_array(k_i4, transformation_round_key(i, k_i1, sbox));
    }
    else {
        new_ki = xor_array(k_i4, k_i1);
    }
    new_ki
}

// Implementation of AddRoundKey
fn add_round_key(state: &mut [[u8; 4]; 4], round_key: &[[u8; 4]; 4]) {
    for i in 0..4 {
        for j in 0..4 {
            state[i][j] ^= round_key[i][j];
        }
    }
}

