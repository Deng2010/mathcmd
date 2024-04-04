use std::fmt::{self, Display};

use crate::libs::complex::Complex;

/// A point in the complex plane.
#[derive(PartialEq, Copy, Clone, Debug, Default)]
pub struct Point {
    /// The complex `x` coordinate.
    pub x: Complex,
    /// The complex `y` coordinate.
    pub y: Complex,
}
impl Point {
    /// Construct a new point with the given complex coordinates.
    pub fn new(x: Complex, y: Complex) -> Self {
        Point { x, y }
    }
    /// Construct a new point with the given coordinates. (Maybe not a complex)
    pub fn new_option(x: Option<&str>, y: Option<&str>) -> Result<Self, String> {
        if y.is_none() {
            return Err("error.need_more_arguments".to_string());
        }
        let (x, y) = (x.unwrap().parse::<Complex>(), y.unwrap().parse::<Complex>());
        if x.is_err() || y.is_err() {
            return Err("error.invalid_argument".to_string());
        }
        Ok(Point::new(x.unwrap(), y.unwrap()))
    }
}
impl Display for Point {
    /// Display the point in the complex plane.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// Create a new point from the given complex coordinates.
#[macro_export]
macro_rules! point {
    ($x: expr, $y: expr) => {
        Point::new($x, $y)
    };
}

/// A line in the complex plane.
#[derive(PartialEq, Copy, Clone, Debug, Default)]
pub struct Line {
    /// The first point of the line.
    pub a: Point,
    /// The second point of the line.
    pub b: Point,
}
impl Line {
    /// Construct a new line with the given points.
    pub fn new(a: Point, b: Point) -> Self {
        Self { a, b }
    }
    /// Construct a new line with the given coordinates. (Maybe not a point)
    pub fn new_option(x: Option<&Point>, y: Option<&Point>) -> Result<Self, String> {
        if x.is_none() || y.is_none() {
            return Err("error.need_more_arguments".to_string());
        }
        Ok(Line::new(x.unwrap().to_owned(), y.unwrap().to_owned()))
    }
    /// Calculate the distance between the two points of the line.
    pub fn dis(&self) -> Complex {
        let sqrt = Complex::sqrt;
        let pow = Complex::pow;
        sqrt(pow(self.a.x - self.b.x, 2) + pow(self.a.y - self.b.y, 2))
    }
}
