pub fn vec_u8_to_u64(input: Vec<u8>) -> u64 {
    let mut block_bytes = [0u8; 8];
    block_bytes[..input.len()].copy_from_slice(input.as_ref());
    u64::from_be_bytes(block_bytes)
}

pub fn u64_to_u8(input: u64) -> Vec<u8> {
    input.to_be_bytes().to_vec()
}

pub fn vec_u8_to_u128(input: Vec<u8>) -> u128 {
    let mut block_bytes = [0u8; 16];
    block_bytes[..input.len()].copy_from_slice(input.as_ref());
    u128::from_be_bytes(block_bytes)
}

pub fn vec_u128_to_u8(input: Vec<u128>) -> Vec<u8> {
    input
        .iter()
        .flat_map(|x| x.to_be_bytes())
        .collect()
}

pub fn u128_to_u8(input: u128) -> Vec<u8> {
    input.to_be_bytes().to_vec()
}

#[test]
fn test_vec_u8_to_u64() {
    let input = vec![0x41, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x10];
    let res = vec_u8_to_u64(input);
    assert_eq!(res, 0x4100001000000010);
}

#[test]
fn test_u64_to_u8() {
    let input = 0x4100001000000010;
    let res = u64_to_u8(input);
    assert_eq!(res, vec![0x41, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x10]);
}

#[test]
fn test_back_and_forth_u64() {
    let input = 0x4100001000000010;
    let res = vec_u8_to_u64(u64_to_u8(input));
    assert_eq!(res, input);
}

#[test]
fn test_back_and_forth_u128() {
    let input = 544281363420033034280345197843107640;
    let res = vec_u8_to_u128(u128_to_u8(input));
    assert_eq!(res, input);
}
