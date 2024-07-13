use std::{
    fmt::{self, Display},
    ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign},
    str::FromStr,
};

#[macro_export]
macro_rules! comp {
    ($re: expr, $im: expr) => {
        Comp::new($re, $im)
    };
    ($re: expr) => {
        Comp::new($re, 0.0)
    };
    () => {
        Comp::default()
    };
}

#[derive(PartialEq, Copy, Clone, Debug, Default)]
pub struct Comp {
    pub re: f64,
    pub im: f64,
}
impl Comp {
    pub fn new(re: f64, im: f64) -> Comp {
        Comp { re, im }
    }
    pub fn to_num(self) -> f64 {
        self.re
    }
    pub fn sqrt(a: Comp) -> Comp {
        if a.im == 0.0 {
            if a.re < 0.0 {
                return comp!(0.0, f64::sqrt(-a.re));
            }
            return comp!(f64::sqrt(a.re));
        }
        let s: f64 = f64::sqrt((f64::sqrt(a.re * a.re + a.im * a.im) + a.re) / 2.0);
        comp!(s, a.im / f64::abs(a.im) * (s - a.re))
    }
    pub fn pow(a: Comp, b: u32) -> Comp {
        match b {
            0 => comp!(1.0),
            1 => a,
            _ if b % 2 == 0 => {
                let k: Comp = Comp::pow(a, b / 2);
                k * k
            }
            _ => {
                let k: Comp = Comp::pow(a, b / 2);
                k * k * a
            }
        }
    }
}

impl Display for Comp {
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

impl Add<Comp> for Comp {
    type Output = Comp;
    fn add(self, rhs: Comp) -> Comp {
        Comp {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl Add<f64> for Comp {
    type Output = Comp;
    fn add(self, rhs: f64) -> Comp {
        self + comp!(rhs)
    }
}

impl Add<Comp> for f64 {
    type Output = Comp;
    fn add(self, rhs: Comp) -> Comp {
        comp!(self) + rhs
    }
}

impl AddAssign<Comp> for Comp {
    fn add_assign(&mut self, rhs: Comp) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

impl AddAssign<f64> for Comp {
    fn add_assign(&mut self, rhs: f64) {
        self.re += rhs;
    }
}

impl Sub<Comp> for Comp {
    type Output = Comp;
    fn sub(self, rhs: Comp) -> Comp {
        Comp {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl Sub<Comp> for f64 {
    type Output = Comp;
    fn sub(self, rhs: Comp) -> Comp {
        comp!(self) - rhs
    }
}

impl Sub<f64> for Comp {
    type Output = Comp;
    fn sub(self, rhs: f64) -> Comp {
        self - comp!(rhs)
    }
}

impl SubAssign<Comp> for Comp {
    fn sub_assign(&mut self, rhs: Comp) {
        self.re -= rhs.re;
        self.im -= rhs.im;
    }
}

impl SubAssign<f64> for Comp {
    fn sub_assign(&mut self, rhs: f64) {
        self.re -= rhs;
    }
}

impl Mul<Comp> for Comp {
    type Output = Comp;
    fn mul(self, rhs: Comp) -> Comp {
        Comp {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.im * rhs.re + self.re * rhs.im,
        }
    }
}

impl Mul<f64> for Comp {
    type Output = Comp;
    fn mul(self, rhs: f64) -> Comp {
        self * comp!(rhs)
    }
}

impl Div<Comp> for Comp {
    type Output = Comp;
    fn div(self, rhs: Comp) -> Comp {
        Comp {
            re: (self.re * rhs.re + self.im * rhs.im) / (rhs.re * rhs.re + rhs.im + rhs.im),
            im: (self.im * rhs.re + self.re * rhs.im) / (rhs.re * rhs.re + rhs.im + rhs.im),
        }
    }
}

impl Div<f64> for Comp {
    type Output = Comp;
    fn div(self, rhs: f64) -> Comp {
        self / comp!(rhs)
    }
}

impl Neg for Comp {
    type Output = Comp;
    fn neg(self) -> Comp {
        comp!(-self.re, -self.im)
    }
}

impl PartialEq<f64> for Comp {
    fn eq(&self, rhs: &f64) -> bool {
        &self.re == rhs
    }
}
#[derive(Debug)]
pub struct ParseComplexError;

impl FromStr for Comp {
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

    use super::Comp;

    pub const CPI: Comp = Comp { re: PI, im: 0.0 };
    pub const CE: Comp = Comp { re: E, im: 0.0 };
    pub const CTAU: Comp = Comp { re: TAU, im: 0.0 };
}
