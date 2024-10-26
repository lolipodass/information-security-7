use primitive_types::U256;
use rand::{ thread_rng, Rng };

use super::number_utils::mod_inverse;

struct KnapsackCipher {
    super_seq: Vec<u128>,
    normal_seq: Vec<u128>,
    a: u128,
    a_inv: u128,
    n: u128,
    size: u8,
}

impl KnapsackCipher {
    fn new(size: u8) -> Self {
        let mut rand = thread_rng();

        let super_seq = Self::superincreasing_sequence(size);
        let n = super_seq.iter().sum::<u128>() + 2;

        let (a, a_inv) = loop {
            let a = rand.gen_range(1..n);
            let a_inv = mod_inverse(a, n);
            if a_inv.is_some() {
                break (a, a_inv.unwrap());
            }
        };

        let normal_seq = Self::normal_sequence(super_seq.clone(), a, n);

        Self {
            super_seq,
            normal_seq,
            a,
            a_inv,
            n,
            size,
        }
    }

    pub fn encrypt(&self, input: Vec<u8>) -> Vec<u128> {
        let mut res = Vec::new();
        for char in input {
            let mut sum = 0u128;
            for j in 0..self.size {
                if ((char >> (self.size - j - 1)) & 1) == 1 {
                    sum += self.normal_seq[j as usize];
                }
            }
            res.push(sum);
        }
        res
    }

    pub fn decrypt(&self, input: Vec<u128>) -> Vec<u8> {
        let mut res = Vec::new();

        for sum in input {
            let mut fixed_sum = Self::multiplication(sum, self.a_inv, self.n);

            let mut elem = 0u8;

            for i in (0..self.size).rev() {
                if fixed_sum <= 0 {
                    break;
                }

                if fixed_sum >= self.super_seq[i as usize] {
                    fixed_sum -= self.super_seq[i as usize];

                    elem |= 1 << (self.size - i - 1);
                }
            }

            res.push(elem);
        }

        res
    }

    fn multiplication(a: u128, b: u128, n: u128) -> u128 {
        ((U256::from(a) * U256::from(b)) % U256::from(n)).as_u128()
    }

    fn normal_sequence(super_sequence: Vec<u128>, a: u128, n: u128) -> Vec<u128> {
        let mut normal_sequence = Vec::with_capacity(super_sequence.len());
        for i in 0..super_sequence.len() {
            normal_sequence.push(Self::multiplication(super_sequence[i], a, n));
        }

        normal_sequence
    }

    fn superincreasing_sequence(amount: u8) -> Vec<u128> {
        let mut res = Vec::new();

        let mut rand = thread_rng();
        let mut sum = 0;
        for i in 1..(amount as u128) + 1 {
            let next_elem =
                sum +
                ((rand.gen_range(100..1000) as u128) << i).pow(rand.gen_range(2..8)) +
                (rand.gen_range(1..1000) as u128);

            res.push(next_elem);

            sum += next_elem;
        }
        res
    }
}

#[test]
fn test_knapsack() {
    let knapsack = KnapsackCipher::new(8);

    let text = "hello".as_bytes().to_vec();
    let enc = knapsack.encrypt(text.clone());
    let dec = knapsack.decrypt(enc);

    assert_eq!(text, dec);
}
