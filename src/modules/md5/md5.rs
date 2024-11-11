use crate::modules::md5::consts::{ K, SHIFTS };

use super::consts::{ A0, B0, C0, D0 };

pub fn md5(data: &[u8]) -> [u8; 16] {
    const CHUNK_SIZE: usize = 64;
    const WORD_SIZE: usize = 4;

    let input = pad(data);

    let mut a = A0;
    let mut b = B0;
    let mut c = C0;
    let mut d = D0;

    for block in input.chunks(CHUNK_SIZE) {
        let mut words = [0u32; 16];
        for i in 0..16 {
            words[i] = u32::from_le_bytes([
                block[i * WORD_SIZE],
                block[i * WORD_SIZE + 1],
                block[i * WORD_SIZE + 2],
                block[i * WORD_SIZE + 3],
            ]);
        }

        for i in 0..64 {
            let a_ = a
                .wrapping_add(f(b, c, d, i))
                .wrapping_add(words[g(i)])
                .wrapping_add(K[i])
                .rotate_left(SHIFTS[i])
                .wrapping_add(b);
            (a, b, c, d) = (d, a_, b, c);
        }
    }

    let mut result = [0u8; 16];
    result[0..4].copy_from_slice(&a.to_le_bytes());
    result[4..8].copy_from_slice(&b.to_le_bytes());
    result[8..12].copy_from_slice(&c.to_le_bytes());
    result[12..16].copy_from_slice(&d.to_le_bytes());

    result
}

fn pad(data: &[u8]) -> Vec<u8> {
    let mut res = data.to_vec();
    let len = res.len() as u64;
    res.push(0x80);

    while res.len() % 64 != 56 {
        res.push(0);
    }

    res.extend(len.to_le_bytes());
    res.to_vec()
}

fn f(b: u32, c: u32, d: u32, i: usize) -> u32 {
    match i {
        0..=15 => (b & c) | (!b & d),
        16..=31 => (d & b) | (!d & c),
        32..=47 => b ^ c ^ d,
        48..=63 => c ^ (b | !d),
        _ => unreachable!(),
    }
}

fn g(i: usize) -> usize {
    match i {
        0..=15 => i,
        16..=31 => (5 * i + 1) % 16,
        32..=47 => (3 * i + 5) % 16,
        48..=63 => (7 * i) % 16,
        _ => unreachable!(),
    }
}

#[test]
fn test_md5() {
    let input = "They are deterministic";

    let expected = "23db6982caef9e9152f1a5b2589e6ca3";
    let result = hex::encode(md5(input.as_bytes()));
    assert_eq!(result, expected);
}
