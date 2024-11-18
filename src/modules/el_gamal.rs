use num_bigint::{ BigInt, BigUint, RandBigInt, ToBigInt };
use num_prime::RandPrime;
use num_traits::One;
use rand::thread_rng;

use crate::modules::number_utils::ceil_to_8;

use super::number_utils::{ mod_inverse_big, primitive_root };

pub struct ElGamal {
    p: BigUint,
    g: BigUint,
    y: BigUint,
    x: BigUint,
    block_size: usize,
    exponent: BigUint,
}

impl ElGamal {
    pub fn new(size: usize) -> Self {
        let mut rand = thread_rng();
        let block_size = ceil_to_8(size);
        let (p, g) = loop {
            let p: BigUint = rand.gen_prime(block_size + 7, None);

            if p.bits() < (block_size as u64) {
                continue;
            }

            let g = primitive_root(p.clone());
            if let Some(g) = g {
                break (p, g);
            }
        };

        let x = rand.gen_biguint_below(&p);
        let y = g.modpow(&x, &p);

        let exponent = &p - &x - &BigUint::one();

        Self {
            p: p.clone(),
            g,
            y,
            x,
            block_size: block_size / 8 + 1,
            exponent,
        }
    }

    pub fn encrypt(&self, text: &[u8]) -> Vec<u8> {
        let encrypted_blocks: Vec<(BigUint, BigUint)> = text
            .chunks(self.block_size - 1)
            .map(|block| self.encrypt_block(BigUint::from_bytes_be(block)))
            .collect();

        let mut res = Vec::new();

        for (a, b) in encrypted_blocks {
            res.append(&mut Self::pad_bytes(&a.to_bytes_be(), self.block_size));
            res.append(&mut Self::pad_bytes(&b.to_bytes_be(), self.block_size));
        }
        res
    }

    pub fn encrypt_block(&self, block: BigUint) -> (BigUint, BigUint) {
        let k = rand::thread_rng().gen_biguint_below(&self.p);

        let a = self.g.modpow(&k, &self.p);
        let y_pow = self.y.modpow(&k, &self.p);
        let b = (block * y_pow) % &self.p;

        (a, b)
    }

    pub fn decrypt(&self, text: &[u8]) -> Vec<u8> {
        let mut res = Vec::new();
        for block in text.chunks(self.block_size * 2) {
            let (a_bytes, b_bytes) = block.split_at(self.block_size);
            res.append(
                &mut self
                    .decrypt_block(BigUint::from_bytes_be(a_bytes), BigUint::from_bytes_be(b_bytes))
                    .to_bytes_be()
            );
        }

        res
    }

    pub fn decrypt_block(&self, a: BigUint, b: BigUint) -> BigUint {
        (b * a.modpow(&self.exponent, &self.p)) % &self.p
    }

    pub fn sign(&self, text: &[u8]) -> (BigUint, BigUint) {
        let hash = BigInt::from_bytes_be(num_bigint::Sign::Plus, blake3::hash(text).as_bytes());

        let (k, k_1) = loop {
            let k = rand::thread_rng().gen_biguint_below(&self.p);

            let k_1 = mod_inverse_big(k.clone().into(), (&self.p - BigUint::one()).into());
            if k_1.is_some() {
                break (k, k_1.unwrap().to_biguint().unwrap());
            }
        };

        let r = self.g.modpow(&k, &self.p);

        let val = &hash - BigInt::from_biguint(num_bigint::Sign::Plus, &self.x * &r);
        let s = (val * k_1.to_bigint().unwrap()) % (&self.p.to_bigint().unwrap() - BigInt::one());

        (r, s.to_biguint().unwrap())
    }

    pub fn verify(&self, text: &[u8], signature: (BigUint, BigUint)) -> bool {
        let (r, s) = signature;

        let hash = BigUint::from_bytes_be(blake3::hash(text).as_bytes());
        let g_pow = self.g.modpow(&hash, &self.p);

        let y_pow = self.y.modpow(&r, &self.p);
        let r_pow = r.modpow(&s, &self.p);

        g_pow == (y_pow * r_pow) % &self.p
    }

    fn pad_bytes(bytes: &[u8], length: usize) -> Vec<u8> {
        let mut padded = vec![0u8; length];
        let offset = length - bytes.len();
        padded[offset..].copy_from_slice(bytes);
        padded
    }
}

#[test]
fn test_elgamal() {
    let text =
        "hi, this is really long text to encrypt; that contains more than one block, and have соме странге текст".as_bytes();
    println!("text {:?}", text);
    let el_gamal = ElGamal::new(100);
    let enc = el_gamal.encrypt(text);
    println!("enc {:?}", enc);
    let dec = el_gamal.decrypt(&enc);

    assert_eq!(text, dec);
}

#[test]
fn test_elgamal_sign() {
    let text =
        "hi, this is really long text to encrypt; that contains more than one block, and have соме странге текст".as_bytes();
    let el_gamal = ElGamal::new(100);
    let signature = el_gamal.sign(text);
    assert!(el_gamal.verify(text, signature));
}
