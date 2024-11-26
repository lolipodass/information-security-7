use crate::modules::number_utils::mod_inverse;

use super::point::Point;

pub struct EpilepticCurve {
    n: i64,
    a: i64,
    b: i64,
    sqrts: Vec<u64>,
}

impl EpilepticCurve {
    pub fn new(n: u64, a: i64, b: i64) -> Self {
        if (4 * a.pow(3) + 27 * b * b) % (n as i64) == 0 {
            panic!("Invalid curve");
        }

        let mut sqrts = Vec::new();
        for y in 0..n {
            sqrts.push(y.pow(2) % n);
        }

        Self { n: n as i64, a, b, sqrts }
    }

    pub fn sum_points(&self, a: Point, b: Point) -> Point {
        match (a, b) {
            (Point::Infinite, _) => b,
            (_, Point::Infinite) => a,
            (Point::Finite { x: a_x, y: a_y }, Point::Finite { x: b_x, y: b_y }) => {
                let lambda = match self.calculate_lambda(a_x, a_y, b_x, b_y) {
                    Some(x) => x.rem_euclid(self.n as i64),
                    None => {
                        return Point::Infinite;
                    }
                };

                let x = lambda.pow(2) - a_x - b_x;
                let y = lambda * (a_x - x) - a_y;

                Point::new(x.rem_euclid(self.n as i64), y.rem_euclid(self.n as i64))
            }
        }
    }

    fn calculate_lambda(&self, a_x: i64, a_y: i64, b_x: i64, b_y: i64) -> Option<i64> {
        if a_x != b_x {
            let inv = mod_inverse((b_x - a_x).rem_euclid(self.n), self.n)?;
            Some((b_y - a_y) * (inv as i64))
        } else {
            let inv = mod_inverse((2 * a_y).rem_euclid(self.n), self.n)?;
            Some((3 * a_x.pow(2) + self.a) * (inv as i64))
        }
    }

    pub fn multiply_point(&self, point: Point, scalar: u64) -> Point {
        let mut scalar = scalar;
        let mut res = Point::Infinite;
        let mut buf = point;
        while scalar > 0 {
            if (scalar & 1) == 1 {
                res = self.sum_points(res, buf);
            }
            scalar >>= 1;
            buf = self.sum_points(buf, buf);
        }

        res
    }

    pub fn find_point_in_range(&self, x_min: i64, x_max: i64) -> Vec<Point> {
        let mut points = Vec::new();
        for x in x_min..x_max {
            let x_pow = (x.pow(3) + self.a * x + self.b) % self.n;
            let sqrts = self.sqrt(x_pow as u64);
            for y in &sqrts {
                points.push(Point::new(x, *y as i64));
            }
        }
        points
    }

    pub fn sqrt(&self, a: u64) -> Vec<u64> {
        self.sqrts
            .clone()
            .into_iter()
            .enumerate()
            .filter_map(|(y, y2)| {
                if y2 == a { Some(y as u64) } else { None }
            })
            .collect()
    }
}

#[test]
fn test_sum_points() {
    let curve = EpilepticCurve::new(751, -1, 1);
    let point1 = Point::new(1, 1);
    let point2 = Point::new(750, 1);
    let point3 = Point::new(45, 31);
    let point4 = Point::new(43, 527);

    assert_eq!(curve.sum_points(point1, point2), Point::new(0, 750));
    assert_eq!(curve.sum_points(point1, point3), Point::new(316, 228));
    assert_eq!(curve.sum_points(point1, point4), Point::new(433, 704));
}

#[test]
fn test_multiply_point() {
    let curve = EpilepticCurve::new(751, -1, 1);
    let point = Point::new(1, 1);

    assert_eq!(curve.multiply_point(point, 2), Point::new(750, 1));
    assert_eq!(curve.multiply_point(point, 3), Point::new(0, 750));
    assert_eq!(curve.multiply_point(point, 4), Point::new(3, 746));
    assert_eq!(curve.multiply_point(point, 5), Point::new(5, 11));
}
