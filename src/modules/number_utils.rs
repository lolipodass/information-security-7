use num_bigint::BigInt;
use num_bigint::BigUint;
use num_bigint::ToBigUint;
use num_prime::nt_funcs::factorize;
use num_traits::One;
use num_traits::Euclid;

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

pub fn mod_inverse<T>(a: T, n: T) -> Option<u128> where T: Copy + Into<i128> + PartialEq {
    let (gcd, x, _) = gcd_bezout(a.into(), n.into());
    if gcd == 1 {
        Some(x.rem_euclid(n.into()) as u128)
    } else {
        None
    }
}

pub fn gcd_bezout(a: i128, b: i128) -> (i128, i128, i128) {
    let mut x = 1;
    let mut y = 0;
    let mut x1 = 0;
    let mut y1 = 1;

    let mut a = a;
    let mut b = b;
    while b != 0 {
        let q = a / b;
        (x, x1) = (x1, x - q * x1);
        (y, y1) = (y1, y - q * y1);
        (a, b) = (b, a - q * b);
    }

    (a, x, y)
}

pub fn mod_inverse_big(a: BigInt, n: BigInt) -> Option<BigInt> {
    let (gcd, x, _) = gcd_bezout_big(a, n.clone());
    if gcd == BigInt::one() {
        Some(x.rem_euclid(&n))
    } else {
        None
    }
}

pub fn gcd_bezout_big(a: BigInt, b: BigInt) -> (BigInt, BigInt, BigInt) {
    let mut x = BigInt::one();
    let mut y = BigInt::ZERO;
    let mut x1 = BigInt::ZERO;
    let mut y1 = BigInt::one();

    let mut a = a;
    let mut b = b;
    while b != BigInt::ZERO {
        let q = &a / &b;
        let temp_x = x.clone();
        let temp_y = y.clone();

        x = x1.clone();
        y = y1.clone();

        x1 = temp_x - &q * &x1;
        y1 = temp_y - &q * &y1;

        let temp_a = a.clone();
        a = b.clone();
        b = temp_a - &q * &b;
    }

    (a.clone(), x.clone(), y.clone())
}

/// Finds the primitive root of a given number.
///
/// # Notes
/// expect that n will be prime
pub fn primitive_root(n: BigUint) -> Option<BigUint> {
    let phi = n.clone() - BigUint::one();
    let factors = factorize(phi.clone());
    let fact = factors
        .iter()
        .map(|x| x.0)
        .collect::<Vec<&BigUint>>();

    for res in num_iter::range((2).to_biguint().unwrap(), n.clone()) {
        let mut ok = true;
        for i in &fact {
            if ok && res.modpow(&(phi.clone() / *i), &n) == BigUint::one() {
                ok = false;
            }
        }
        if ok {
            return Some(res);
        }
    }

    None
}

pub fn ceil_to_8(input: usize) -> usize {
    ((input + 7) / 8) * 8
}

#[test]
fn test_round_to_8() {
    assert_eq!(ceil_to_8(8), 8);
    assert_eq!(ceil_to_8(9), 16);
    assert_eq!(ceil_to_8(10), 16);
    assert_eq!(ceil_to_8(11), 16);
    assert_eq!(ceil_to_8(12), 16);
}

#[test]
fn test_primitive_root() {
    assert_eq!(primitive_root((5).to_biguint().unwrap()), Some((2).to_biguint().unwrap()));
    assert_eq!(primitive_root((7).to_biguint().unwrap()), Some((3).to_biguint().unwrap()));
    assert_eq!(primitive_root((11).to_biguint().unwrap()), Some((2).to_biguint().unwrap()));
}

#[test]
fn test_gcd_bezout() {
    assert_eq!(gcd_bezout(10, 15), (5, -1, 1));
    assert_eq!(gcd_bezout(31, 17), (1, -6, 11));
    assert_eq!(gcd_bezout(3, 10), (1, -3, 1));
}

#[test]
fn test_mod_inverse() {
    assert_eq!(mod_inverse(3, 11), Some(4));
    assert_eq!(mod_inverse(3, 10), Some(7));
    assert_eq!(mod_inverse(3, 9), None);
}
