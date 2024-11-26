#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Point {
    Finite {
        x: i64,
        y: i64,
    },
    Infinite,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Point::Finite { x, y }
    }

    pub fn infinite() -> Self {
        Point::Infinite
    }
}
