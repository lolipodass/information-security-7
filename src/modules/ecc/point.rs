use std::fmt;

use serde::{ Deserialize, Serialize };

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
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
    pub fn negative(&self) -> Self {
        match self {
            Point::Finite { x, y } => Point::Finite { x: *x, y: -y },
            Point::Infinite => Point::Infinite,
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Infinite => { write!(f, "Infinite") }
            Self::Finite { x, y } => { write!(f, "({}, {})", x, y) }
        }
    }
}
