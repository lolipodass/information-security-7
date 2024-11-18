use blake3::Hasher;
use num_bigint::{ BigUint, RandBigInt };
use num_iter::range_inclusive;
use num_prime::RandPrime;
use num_traits::One;

pub struct SchnorrSignature {
    p: BigUint,
    q: BigUint,
    g: BigUint,
    y: BigUint,
    w: BigUint,
}

impl SchnorrSignature {
    pub fn new(size: usize) -> Self {
        let mut rng = rand::thread_rng();
        let p: BigUint = rng.gen_prime(size, None);
        let q = loop {
            let q: BigUint = rng.gen_prime((p.bits() - 1) as usize, None);
            if (&p - BigUint::one()) % &q == BigUint::ZERO {
                break q;
            }
        };

        let mut g: BigUint = BigUint::one();
        for candidate in range_inclusive(BigUint::from(2u8), p.clone()) {
            if candidate.modpow(&q, &p) == BigUint::one() {
                g = candidate;
                break;
            }
        }

        let w = rng.gen_biguint_below(&q);
        let y = g.modpow(&(&q - &w), &p);

        SchnorrSignature { p, q, g, w, y }
    }

    pub fn schnorr_sign(&self, text: &[u8]) -> (BigUint, BigUint) {
        let mut rng = rand::thread_rng();
        let r = rng.gen_biguint_below(&self.q);

        let x = self.g.modpow(&r, &self.p);
        let mut hasher = Hasher::new();
        hasher.update(text);
        hasher.update(&x.to_bytes_be());

        let s1 = BigUint::from_bytes_be(hasher.finalize().as_bytes());
        let s2 = (r + &s1 * &self.w) % &self.q;
        (s1, s2)
    }

    pub fn schnorr_verify(&self, text: &[u8], signature: (BigUint, BigUint)) -> bool {
        let (s1, s2) = signature;
        let g_pow = self.g.modpow(&s2, &self.p);
        let y_pow = self.y.modpow(&s1, &self.p);

        let x = (g_pow * y_pow) % &self.p;

        let mut hasher = Hasher::new();
        hasher.update(text);
        hasher.update(&x.to_bytes_be());

        let h = BigUint::from_bytes_be(hasher.finalize().as_bytes());

        h == s1
    }
}

#[test]
fn test_schnorr_signature() {
    let text = "hello".as_bytes();
    let schnorr_signature = SchnorrSignature::new(20);
    let signature = schnorr_signature.schnorr_sign(text);
    assert!(schnorr_signature.schnorr_verify(text, signature));
}
