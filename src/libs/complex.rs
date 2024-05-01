use std::{
    fmt::{self, Display},
    ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign},
    str::FromStr,
};

#[macro_export]
macro_rules! comp {
    ($re: expr, $im: expr) => {
        Complex::new($re, $im)
    };
    ($re: expr) => {
        Complex::new($re, 0.0)
    };
    () => {
        Complex::default()
    };
}

#[derive(PartialEq, Copy, Clone, Debug, Default)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}
impl Complex {
    pub fn new(re: f64, im: f64) -> Complex {
        Complex { re, im }
    }
    pub fn to_num(self) -> f64 {
        self.re
    }
    pub fn sqrt(a: Complex) -> Complex {
        if a.im == 0.0 {
            if a.re < 0.0 {
                return comp!(0.0, f64::sqrt(-a.re));
            }
            return comp!(f64::sqrt(a.re));
        }
        let s: f64 = f64::sqrt((f64::sqrt(a.re * a.re + a.im * a.im) + a.re) / 2.0);
        comp!(s, a.im / f64::abs(a.im) * (s - a.re))
    }
    pub fn pow(a: Complex, b: u32) -> Complex {
        match b {
            0 => comp!(1.0),
            1 => a,
            _ if b % 2 == 0 => {
                let k: Complex = Complex::pow(a, b / 2);
                k * k
            }
            _ => {
                let k: Complex = Complex::pow(a, b / 2);
                k * k * a
            }
        }
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res: String = String::new();
        if self.re != 0.0 {
            res = self.re.to_string();
            if self.im > 0.0 {
                res += "+";
            }
        }
        if self.im == 1.0 {
            res += "i";
        } else if self.im != 0.0 {
            res = res + self.im.to_string().as_str() + "i";
        } else if self.re == 0.0 {
            res = String::from("0");
        }
        write!(f, "{res}")
    }
}

impl Add<Complex> for Complex {
    type Output = Complex;
    fn add(self, rhs: Complex) -> Complex {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl Add<f64> for Complex {
    type Output = Complex;
    fn add(self, rhs: f64) -> Complex {
        self + comp!(rhs)
    }
}

impl Add<Complex> for f64 {
    type Output = Complex;
    fn add(self, rhs: Complex) -> Complex {
        comp!(self) + rhs
    }
}

impl AddAssign<Complex> for Complex {
    fn add_assign(&mut self, rhs: Complex) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

impl AddAssign<f64> for Complex {
    fn add_assign(&mut self, rhs: f64) {
        self.re += rhs;
    }
}

impl Sub<Complex> for Complex {
    type Output = Complex;
    fn sub(self, rhs: Complex) -> Complex {
        Complex {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl Sub<Complex> for f64 {
    type Output = Complex;
    fn sub(self, rhs: Complex) -> Complex {
        comp!(self) - rhs
    }
}

impl Sub<f64> for Complex {
    type Output = Complex;
    fn sub(self, rhs: f64) -> Complex {
        self - comp!(rhs)
    }
}

impl SubAssign<Complex> for Complex {
    fn sub_assign(&mut self, rhs: Complex) {
        self.re -= rhs.re;
        self.im -= rhs.im;
    }
}

impl SubAssign<f64> for Complex {
    fn sub_assign(&mut self, rhs: f64) {
        self.re -= rhs;
    }
}

impl Mul<Complex> for Complex {
    type Output = Complex;
    fn mul(self, rhs: Complex) -> Complex {
        Complex {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.im * rhs.re + self.re * rhs.im,
        }
    }
}

impl Mul<f64> for Complex {
    type Output = Complex;
    fn mul(self, rhs: f64) -> Complex {
        self * comp!(rhs)
    }
}

impl Div<Complex> for Complex {
    type Output = Complex;
    fn div(self, rhs: Complex) -> Complex {
        Complex {
            re: (self.re * rhs.re + self.im * rhs.im) / (rhs.re * rhs.re + rhs.im + rhs.im),
            im: (self.im * rhs.re + self.re * rhs.im) / (rhs.re * rhs.re + rhs.im + rhs.im),
        }
    }
}

impl Div<f64> for Complex {
    type Output = Complex;
    fn div(self, rhs: f64) -> Complex {
        self / comp!(rhs)
    }
}

impl Neg for Complex {
    type Output = Complex;
    fn neg(self) -> Complex {
        comp!(-self.re, -self.im)
    }
}

impl PartialEq<f64> for Complex {
    fn eq(&self, rhs: &f64) -> bool {
        &self.re == rhs
    }
}
#[derive(Debug)]
pub struct ParseComplexError;

impl FromStr for Complex {
    type Err = ParseComplexError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains('+') {
            let (x, y) = s
                .strip_suffix('i')
                .and_then(|str| str.split_once('+'))
                .ok_or(ParseComplexError)?;

            Ok(comp!(
                x.parse().map_err(|_| ParseComplexError)?,
                y.parse().map_err(|_| ParseComplexError)?
            ))
        } else if s == "i" {
            Ok(comp!(0.0, 1.0))
        } else if s.ends_with('i') {
            Ok(comp!(
                0.0,
                s.strip_suffix('i')
                    .unwrap()
                    .parse()
                    .map_err(|_| ParseComplexError)?
            ))
        } else {
            Ok(comp!(s.parse().map_err(|_| ParseComplexError)?))
        }
    }
}

pub mod consts {
    use std::f64::consts::{E, PI, TAU};

    use super::Complex;

    pub const CPI: Complex = Complex { re: PI, im: 0.0 };
    pub const CE: Complex = Complex { re: E, im: 0.0 };
    pub const CTAU: Complex = Complex { re: TAU, im: 0.0 };
}
