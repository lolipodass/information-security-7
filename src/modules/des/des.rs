#![allow(warnings)]
use std::{ mem::swap, ops::Range, vec };

use super::consts::{
    EXPAND,
    F_PERMUTATION,
    INITIAL_KEY_PERMUTATION,
    KEY_PERMUTATION,
    KEY_SHIFTS,
    BLOCK_PERMUTATIONS,
    SBOXES,
};

pub fn encrypt_des(input: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {
    compute(input, key, false)
}

pub fn decrypt_des(input: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {
    compute(input, key, true)
}

fn compute(input: &Vec<u8>, key: &Vec<u8>, decrypt: bool) -> Vec<u8> {
    let mut res = Vec::new();

    //key generation
    let key = permutation(u8_to_u64(key.clone()), &INITIAL_KEY_PERMUTATION);

    let mut l_key = (key >> 36) & 0xff_ffff;
    let mut r_key = (key >> 8) & 0xff_ffff;

    let mut keys = Vec::new();

    for i in 0..16 {
        l_key = shift_left(l_key, KEY_SHIFTS[i]);
        r_key = shift_left(r_key, KEY_SHIFTS[i]);

        let mut key_i = (l_key << 36) | (r_key << 8);

        key_i = permutation(key_i, &KEY_PERMUTATION);

        keys.push(key_i >> 16);
    }

    for block in input.chunks(8) {
        let mut block_value = u8_to_u64(block.to_vec());

        block_value = permutation(block_value, &BLOCK_PERMUTATIONS[0]);

        let mut l_block = (block_value >> 32) & 0xffffffff;
        let mut r_block = block_value & 0xffffffff;

        let key_range: Vec<usize> = if decrypt {
            (0..16).rev().collect()
        } else {
            (0..16).collect()
        };

        //round
        for i in key_range {
            l_block ^= f_function(r_block, *keys.get(i).expect("out of bounds"));

            //possible swap only pointers
            swap(&mut l_block, &mut r_block);
        }

        //final permutation
        block_value = (r_block << 32) | l_block;

        block_value = permutation(block_value, &BLOCK_PERMUTATIONS[1]);

        res.append(&mut u64_to_u8(block_value));
    }

    if let Some(last_non_zero_index) = res.iter().rposition(|&x| x != 0) {
        res.truncate(last_non_zero_index + 1);
    }
    res
}

fn f_function(block: u64, key: u64) -> u64 {
    let block_size = 6;

    let block = permutation(block << 32, &EXPAND) >> 16;

    let mut xor = block ^ key;

    let mut res = 0u64;

    for i in 0..8 {
        let index = (xor >> ((7 - i) * block_size)) & 0b111111u64;
        let elem = SBOXES[i][index as usize];
        res |= (elem as u64) << ((7 - i) * 4);
    }

    permutation(res << 32, &F_PERMUTATION) >> 32
}

fn u8_to_u64(input: Vec<u8>) -> u64 {
    let mut block_bytes = [0u8; 8];
    block_bytes[..input.len()].copy_from_slice(input.as_ref());
    u64::from_be_bytes(block_bytes)
}

fn u64_to_u8(input: u64) -> Vec<u8> {
    input.to_be_bytes().to_vec()
}

fn permutation(input: u64, table: &[u8]) -> u64 {
    let mut res = 0u64;

    table
        .iter()
        .enumerate()
        .for_each(|(i, pos)| {
            //first magic number 64 because indexes in table starts from 1, second one 63 because i starts from 0
            res |= ((input >> (64 - pos)) & 1u64) << (63 - i);
        });

    res
}

fn shift_left(key: u64, shift: u8) -> u64 {
    let mut res = key;
    for _ in 0..shift {
        res = (res << 1) | ((res >> 27) & 1);
    }
    res & 0xfff_ffff
}

#[test]
fn test_des() {
    let text = "testi".as_bytes().to_vec();
    let key = "besti".as_bytes().to_vec();

    let encrypted = encrypt_des(&text, &key);
    let decrypted = decrypt_des(&encrypted, &key);
    assert_eq!(text, decrypted);
}

#[test]
fn test_permutation() {
    let res = permutation(
        0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0100_0000u64,
        &BLOCK_PERMUTATIONS[0]
    );
    assert_eq!(
        res,
        0b1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000u64
    );
}

#[test]
fn test_u64to_u8() {
    let input = 0x4100000000000000u64;

    println!("input: {:X}", input);
    println!("input: {:?}", input.to_be_bytes());
    let res = u64_to_u8(input);
    assert_eq!(res, vec![0x41, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
}
