use std::fmt::{self, Display};

use crate::complex::Complex;

#[derive(PartialEq, Copy, Clone, Debug, Default)]
pub struct Point {
    pub x: Complex,
    pub y: Complex,
}
impl Point {
    pub fn new(x: Complex, y: Complex) -> Self {
        Point { x, y }
    }
}
impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[macro_export]
macro_rules! point {
    ($x: expr, $y: expr) => {
        Point::new($x, $y)
    };
}
