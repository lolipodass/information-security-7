use num_bigint::{ BigUint, RandBigInt };
use num_prime::RandPrime;
use num_traits::One;
use rand::thread_rng;

use crate::modules::number_utils::ceil_to_8;

use super::number_utils::primitive_root;

pub struct ElGamal {
    p: BigUint,
    g: BigUint,
    x: BigUint,
    y: BigUint,
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
            x,
            y,
            block_size: block_size / 8,
            exponent,
        }
    }

    pub fn encrypt(&self, text: Vec<u8>) -> Vec<(BigUint, BigUint)> {
        text.chunks(self.block_size)
            .map(|block| self.encrypt_block(BigUint::from_bytes_be(block)))
            .collect()
    }

    pub fn encrypt_block(&self, block: BigUint) -> (BigUint, BigUint) {
        let k = rand::thread_rng().gen_biguint_below(&self.p);

        let a = self.g.modpow(&k, &self.p);
        let y_pow = self.y.modpow(&k, &self.p);
        let b = (block * y_pow) % &self.p;

        (a, b)
    }

    pub fn decrypt(&self, text: Vec<(BigUint, BigUint)>) -> Vec<u8> {
        text.into_iter()
            .flat_map(|(a, b)| self.decrypt_block(a, b).to_bytes_be())
            .collect()
    }

    pub fn decrypt_block(&self, a: BigUint, b: BigUint) -> BigUint {
        (b * a.modpow(&self.exponent, &self.p)) % &self.p
    }
}

#[test]
fn test_elgamal() {
    let text =
        "hi, this is really long text to encrypt; that contains more than one block, and have соме странге текст"
            .as_bytes()
            .to_vec();
    println!("text {:?}", text);
    let el_gamal = ElGamal::new(100);
    let enc = el_gamal.encrypt(text.clone());
    println!("enc {:?}", enc);
    let dec = el_gamal.decrypt(enc);

    assert_eq!(text, dec);
}