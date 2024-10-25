pub fn bbs(n: u16, seed: u16, amount: u32) -> String {
    let n: u32 = n.into();
    let mut res = String::with_capacity(amount as usize);

    let mut prev = (seed as u32).pow(2) % n;

    for _ in 0..amount {
        prev = (prev as u32).pow(2) % n;
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
