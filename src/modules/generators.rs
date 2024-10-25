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

pub fn rc4(text: Vec<u8>, n: u8, key: Vec<u8>) -> Vec<u8> {
    let mut res = Vec::new();
    let max = 1 << n;

    let mut table_s: Vec<usize> = (0..max).collect();
    let mut table_k = vec![0usize; max as usize];

    for i in 0..max {
        table_k[i] = key[i % key.len()] as usize;
    }

    let mut j = 0;
    for i in 0..max {
        j = (j + table_s[i] + table_k[i]) % max;
        table_s.swap(i, j.into());
    }

    let mut i = 0;
    let mut j = 0;

    for char in text {
        i = (i + 1) % max;
        j = (j + table_s[i]) % max;
        table_s.swap(i, j);
        let a = (table_s[i] + table_s[j]) % max;

        res.push(char ^ (table_s[a] as u8));
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

#[test]
fn test_rc4() {
    let text = "hello".as_bytes().to_vec();
    let key = vec![61, 60, 23, 22, 60, 61];
    let encrypted = rc4(text.clone(), 6, key.clone());
    let decrypted = rc4(encrypted, 6, key);

    assert_eq!(decrypted, text);
}
