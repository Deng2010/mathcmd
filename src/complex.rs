use std::{
    ops::{Add, AddAssign, Div, Mul, Neg, Sub},
    str::FromStr,
};

#[macro_export]
macro_rules! comp {
    ($act: expr, $ima: expr) => {
        Complex::new($act, $ima)
    };
    ($act: expr) => {
        Complex::new($act, 0.0)
    };
    () => {
        Complex::new(0.0, 0.0)
    };
}
#[derive(PartialEq, Copy, Clone, Debug, Default)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}
impl Complex {
    pub fn new(rea: f64, ima: f64) -> Complex {
        Complex { re: rea, im: ima }
    }
    pub fn to_num(&mut self) -> f64 {
        self.re
    }
    pub fn preprocess(s: String) -> String {
        let mut str: String = String::new();
        if s.contains("+") {
            if s.ends_with("+i") {
                str += s.strip_suffix("+i").unwrap();
                str += "+1i";
            } else {
                str += &s.clone();
            }
        } else if s.ends_with("i") {
            if s.starts_with("i") {
                str += "0+1i";
            } else {
                str += "0+";
                str += &s.clone();
            }
        } else {
            str += &s.clone();
            str += "+0i";
        }
        str
    }
    pub fn to_string(&self) -> String {
        let mut res: String = String::new();
        if self.re != 0.0 {
            res = self.re.to_string();
            if self.im > 0.0 {
                res = res + "+";
            }
        }
        if self.im == 1.0 {
            res = res + "i";
        } else if self.im != 0.0 {
            res = res + &self.im.to_string() + "i";
        } else if self.re == 0.0 {
            res = "0".to_owned();
        }
        res
    }
    pub fn sqrt(a: Complex) -> Complex {
        if a.im == 0.0 {
            if a.re < 0.0 {
                return comp!(0.0, f64::sqrt(-a.re));
            }
            return comp!(f64::sqrt(a.re));
        }
        let s = f64::sqrt((f64::sqrt(a.re * a.re + a.im * a.im) + a.re) / 2.0);
        return comp!(s, a.im / f64::abs(a.im) * (s - a.re));
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
        return Complex::pow(a, b / 2) * Complex::pow(a, b / 2) * a;
    }
}

#[derive(Debug)]
pub struct ParseComplexError;

impl Add<Complex> for Complex {
    type Output = Complex;
    fn add(self, rhs: Complex) -> Complex {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}
impl AddAssign<Complex> for Complex {
    fn add_assign(&mut self, rhs: Complex) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}
impl Add<f64> for Complex {
    type Output = Complex;
    fn add(self, rhs: f64) -> Complex {
        Complex {
            re: self.re + rhs,
            im: self.im,
        }
    }
}
impl AddAssign<f64> for Complex {
    fn add_assign(&mut self, rhs: f64) {
        self.re += rhs;
    }
}
impl Add<Complex> for f64 {
    type Output = Complex;
    fn add(self, rhs: Complex) -> Complex {
        Complex {
            re: self + rhs.re,
            im: rhs.im,
        }
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
        Complex {
            re: self - rhs.re,
            im: -rhs.im,
        }
    }
}

impl Sub<f64> for Complex {
    type Output = Complex;
    fn sub(self, rhs: f64) -> Complex {
        Complex {
            re: self.re - rhs,
            im: self.im,
        }
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
        Complex {
            re: self.re * rhs,
            im: self.im * rhs,
        }
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
        Complex {
            re: self.re / rhs,
            im: self.im / rhs,
        }
    }
}

impl Neg for Complex {
    type Output = Complex;
    fn neg(self) -> Complex {
        Complex {
            re: -self.re,
            im: -self.im,
        }
    }
}

impl FromStr for Complex {
    type Err = ParseComplexError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let str: &str = &Complex::preprocess(s.to_owned());
        let (x, y) = str
            .strip_suffix('i')
            .and_then(|str| str.split_once('+'))
            .ok_or(ParseComplexError)?;

        let x_fromstr = x.parse::<f64>().map_err(|_| ParseComplexError)?;
        let y_fromstr = y.parse::<f64>().map_err(|_| ParseComplexError)?;

        Ok(Complex {
            re: x_fromstr,
            im: y_fromstr,
        })
    }
}
