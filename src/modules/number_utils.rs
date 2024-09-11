pub fn calculate_gcd(a: i32, b: i32) -> i32 {
    let mut a = a;
    let mut b = b;
    while a != b {
        if a > b {
            a -= b;
        } else {
            b -= a;
        }
    }

    a
}

pub fn find_prime_numbers(a: i32, b: i32) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::new();

    for elem in a..b + 1 {
        res.push(elem);
        for check in 2..((elem as f32).sqrt() as i32) + 1 {
            if elem % check == 0 {
                res.pop();
                break;
            }
        }
    }

    res
}
