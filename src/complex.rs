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

pub fn to_compstr(s: &str) -> String {
    let str: String;
    if s.ends_with("+i") {
        str = s.replace('i', "1i");
    } else if s.contains('+') {
        str = String::from(s);
    } else if s.ends_with('i') {
        str = String::from("0+") + if s == "i" { "1i" } else { s };
    } else {
        str = String::from(s) + "+0i";
    }
    str
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
        let s = f64::sqrt((f64::sqrt(a.re * a.re + a.im * a.im) + a.re) / 2.0);
        comp!(s, a.im / f64::abs(a.im) * (s - a.re))
    }
    pub fn pow(a: Complex, b: u32) -> Complex {
        if b == 0 {
            return comp!(1.0);
        }
        if b == 1 {
            return a;
        }
        if b % 2 == 0 {
            return Complex::pow(a, b / 2) * Complex::pow(a, b / 2);
        }
        Complex::pow(a, b / 2) * Complex::pow(a, b / 2) * a
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
        let s: String = to_compstr(s);
        let (x, y) = s
            .strip_suffix('i')
            .and_then(|str| str.split_once('+'))
            .ok_or(ParseComplexError)?;

        let x_fromstr: f64 = x.parse().map_err(|_| ParseComplexError)?;
        let y_fromstr: f64 = y.parse().map_err(|_| ParseComplexError)?;

        Ok(comp!(x_fromstr, y_fromstr))
    }
}
