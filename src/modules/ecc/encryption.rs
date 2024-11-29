use rand::{ thread_rng, Rng };

use super::{ curve::EpilepticCurve, point::Point };

struct ECCEncryption {
    curve: EpilepticCurve,
    generator: Point,
    d: u64,
    order: u64,
    open_key: Point,
}

impl ECCEncryption {
    fn new(a: i64, b: i64, n: u64, generator: Point) -> Self {
        let curve = EpilepticCurve::new(n, a, b);
        let order = curve.order(generator);
        let d = thread_rng().gen_range(0..order - 1);
        let y = curve.scalar(generator, d);
        Self { curve, generator, d, order, open_key: y }
    }

    fn encrypt_point(&self, message: Point) -> (Point, Point) {
        let mut rand = thread_rng();

        let k = rand.gen_range(0..self.order - 1);

        let c1 = self.curve.scalar(self.generator, k);
        let c2 = self.curve.add(message, self.curve.scalar(self.open_key, k));

        (c1, c2)
    }

    fn decrypt_point(&self, c1: Point, c2: Point) -> Point {
        self.curve.add(c2, -self.curve.scalar(c1, self.d))
    }
}

#[test]
fn test_ecc_encryption() {
    let crypt = ECCEncryption::new(-1, 1, 751, Point::Finite { x: 0, y: 1 });

    let message = Point::Finite { x: 1, y: 6 };

    let (c1, c2) = crypt.encrypt_point(message);
    println!("c1: {}, c2: {}", c1, c2);
    let res = crypt.decrypt_point(c1, c2);

    println!("res: {}", res);
    assert_eq!(res, message);
}
