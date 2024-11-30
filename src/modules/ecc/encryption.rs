use blake3::hash;
use rand::{ thread_rng, Rng };

use crate::modules::number_utils::mod_inverse;

use super::{ curve::EpilepticCurve, point::Point };

pub struct ECCEncryption {
    curve: EpilepticCurve,
    generator: Point,
    d: u64,
    order: u64,
    open_key: Point,
}

impl ECCEncryption {
    pub fn new(a: i64, b: i64, n: u64, generator: Point) -> Self {
        let curve = EpilepticCurve::new(n, a, b);
        let order = curve.order(generator);
        let d = thread_rng().gen_range(2..order - 1);
        let y = curve.scalar(generator, d);
        Self { curve, generator, d, order, open_key: y }
    }

    pub fn encrypt_point_elgamal(&self, message: Point) -> (Point, Point) {
        let mut rand = thread_rng();

        let k = rand.gen_range(2..self.order - 1);

        let c1 = self.curve.scalar(self.generator, k);
        let c2 = self.curve.add(message, self.curve.scalar(self.open_key, k));

        (c1, c2)
    }

    pub fn decrypt_point_elgamal(&self, c1: Point, c2: Point) -> Point {
        self.curve.add(c2, -self.curve.scalar(c1, self.d))
    }

    pub fn sign(&self, text: &[u8]) -> (u64, u64) {
        let hash = self.hash(text);

        loop {
            let (r, k) = match self.generate_random_r_k() {
                Some(r_k) => { r_k }
                None => {
                    continue;
                }
            };

            let k_1 = match mod_inverse(k, self.order) {
                Some(k1) => { k1 as u64 }
                None => {
                    continue;
                }
            };

            let buf = self.d * r + hash;
            let s = (k_1 * buf) % self.order;

            if s != 0 {
                if mod_inverse(s, self.order).is_none() {
                    continue;
                }
                return (r, s);
            }
        }
    }

    pub fn verify(&self, text: &[u8], signature: (u64, u64)) -> bool {
        let (r, s) = signature;

        let hash = self.hash(text);

        let w = match mod_inverse(s, self.order) {
            Some(w) => { w as u64 }
            None => {
                return false;
            }
        };
        let u1 = (hash * w) % self.order;
        let u2 = (r * w) % self.order;

        let u1_g = self.curve.scalar(self.generator, u1);
        let u2_q = self.curve.scalar(self.open_key, u2);

        let point = self.curve.add(u1_g, u2_q);

        match point {
            Point::Finite { x, y: _ } => (x as u64) % self.order == r,
            Point::Infinite => false,
        }
    }

    fn generate_random_r_k(&self) -> Option<(u64, u64)> {
        let mut rand = thread_rng();

        for _ in 0..self.order {
            let k = rand.gen_range(2..self.order - 1);
            if let Point::Finite { x, y: _ } = self.curve.scalar(self.generator, k) {
                let r = (x as u64) % self.order;
                if r != 0 {
                    return Some((r, k));
                }
            }
        }
        None
    }

    fn hash(&self, message: &[u8]) -> u64 {
        let hash = hash(message);
        u64::from_be_bytes(
            hash.as_bytes()[..8].try_into().expect("blake expected to return more that 8 bytes")
        ) % self.order
    }
}

#[test]
fn test_ecc_encryption() {
    let crypt = ECCEncryption::new(-1, 1, 751, Point::Finite { x: 0, y: 1 });

    let message = Point::Finite { x: 1, y: 6 };

    let (c1, c2) = crypt.encrypt_point_elgamal(message);
    println!("c1: {}, c2: {}", c1, c2);
    let res = crypt.decrypt_point_elgamal(c1, c2);

    println!("res: {}", res);
    assert_eq!(res, message);
}

#[test]
fn test_ecc_signature() {
    let crypt = ECCEncryption::new(-1, 1, 751, Point::Finite { x: 0, y: 1 });
    let message = b"Hello, world!";

    let mut signature = crypt.sign(message);
    assert!(crypt.verify(message, signature));

    let message = b"Hello. world!";
    assert!(!crypt.verify(message, signature));

    signature.0 += 1;
    assert!(!crypt.verify(message, signature));
}
