pub fn bbs(n: u8, seed: u8, amount: u32) -> String {
    let mut res = String::new();
    let n: u16 = n.into();

    let mut prev = (seed as u16).pow(2).rem_euclid(n);

    for _ in 0..amount {
        prev = (prev as u16).pow(2).rem_euclid(n);
        res.push_str(&(prev & 1).to_string());
    }

    res
}

#[test]
fn test_bbs() {
    let test1 = bbs(2, 1, 10);
    assert_eq!(test1, "1111111111");

    let test2 = bbs(209, 3, 10);
    assert_eq!(test2, "1000001011");
}
