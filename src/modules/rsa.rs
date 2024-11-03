use num_bigint::{ BigUint, RandBigInt, ToBigInt };
use num_prime::RandPrime;
use num_traits::One;
use rand::{ thread_rng, Rng };

use super::number_utils::{ ceil_to_8, mod_inverse_big };

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
struct RSA {
    p: BigUint,
    q: BigUint,
    n: BigUint,
    e: BigUint,
    d: BigUint,
    block_size: usize,
}

impl Default for RSA {
    fn default() -> Self {
        Self {
            p: BigUint::ZERO,
            q: BigUint::ZERO,
            n: BigUint::ZERO,
            e: BigUint::ZERO,
            d: BigUint::ZERO,
            block_size: 0,
        }
    }
}
impl RSA {
    pub fn new(key_size: usize) -> Self {
        let key_size = ceil_to_8(key_size);
        let mut rand = thread_rng();

        // generate p and q until we find e and d that are not equal
        loop {
            let (p, q) = Self::generate_primes(&mut rand, key_size);
            let n = &p * &q;
            let phi = (&p - 1u8) * (&q - 1u8);

            if let Some((e, d)) = Self::generate_keys(&mut rand, &phi) {
                return RSA {
                    p,
                    q,
                    n,
                    e,
                    d,
                    block_size: key_size / 8,
                };
            }
        }
    }

    pub fn encrypt(&self, text: &[u8]) -> Vec<u8> {
        let mut res = Vec::with_capacity(text.len());

        for block in text.chunks(self.block_size) {
            let mut val = BigUint::from_bytes_be(block).modpow(&self.e, &self.n).to_bytes_be();

            while val.len() <= self.block_size {
                val.insert(0, 0);
            }
            res.append(&mut val);
        }

        res
    }

    pub fn decrypt(&self, encrypted: &[u8]) -> Vec<u8> {
        encrypted
            .chunks(self.block_size + 1)
            .flat_map(|block| {
                let val = BigUint::from_bytes_be(block).modpow(&self.d, &self.n);
                val.to_bytes_be()
            })
            .collect()
    }
    fn generate_primes<R: Rng>(rand: &mut R, key_size: usize) -> (BigUint, BigUint) {
        let p = rand.gen_prime_exact(key_size / 2, None);
        // Generate q slightly larger to ensure n is adequately sized
        let q = rand.gen_prime_exact(key_size / 2 + 1, None);
        (p, q)
    }

    fn generate_keys<R: Rng>(rand: &mut R, phi: &BigUint) -> Option<(BigUint, BigUint)> {
        const MAX_ATTEMPTS: usize = 100;
        let mut attempts = 0;

        while attempts < MAX_ATTEMPTS {
            attempts += 1;
            let e = rand.gen_biguint_range(&BigUint::one(), phi);
            if let Some(d) = mod_inverse_big(e.to_bigint().unwrap(), phi.to_bigint().unwrap()) {
                if e != d.to_biguint().unwrap() {
                    return Some((e, d.to_biguint().unwrap()));
                }
            }
        }
        None
    }
}

#[test]
fn test_rsa() {
    for i in 0..2048 {
        println!("iteration:{}", i);

        let text = "hello this is some long text to get more that one block of data"
            .as_bytes()
            .to_vec();

        let rsa = RSA::new(200);

        let enc = rsa.encrypt(&text);
        let dec = rsa.decrypt(&enc);

        assert_eq!(text, dec);
    }
}
