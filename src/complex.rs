use std::ops::{Add, Div, Mul, Neg, Sub};
#[derive(PartialEq, Copy, Clone, Debug, Default)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}
impl Complex {
    pub fn new(rea: f64, ima: f64) -> Self {
        Self { re: rea, im: ima }
    }
    pub fn to_num(&mut self) -> f64 {
        self.re
    }
    pub fn to_string(&self) -> String {
        let mut res: String = String::new();
        if self.re != 0.0 {
            res = self.re.to_string();
            if self.im == 1.0 {
                res = res + "+i";
            } else if self.im != 0.0 {
                res = res + "+" + &self.im.to_string() + "i";
            }
        } else if self.im == 1.0 {
            res = res + "i";
        } else if self.im != 0.0 {
            res = res + &self.im.to_string() + "i";
        } else {
            res = "0".to_owned();
        }
        res
    }
}
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
// macro_rules! comp_zero {
//
// }
// macro_rules! comp_double_zero {
//
// }
impl Add<Self> for Complex {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}
impl Add<f64> for Complex {
    type Output = Self;
    fn add(self, rhs: f64) -> Self {
        Self {
            re: self.re + rhs,
            im: self.im,
        }
    }
}

impl Sub<Self> for Complex {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl Sub<f64> for Complex {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self {
        Self {
            re: self.re - rhs,
            im: self.im,
        }
    }
}

impl Mul<Self> for Complex {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.im * rhs.re + self.re * rhs.im,
        }
    }
}

impl Mul<f64> for Complex {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self {
            re: self.re * rhs,
            im: self.im * rhs,
        }
    }
}

impl Div<Self> for Complex {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Self {
            re: (self.re * rhs.re + self.im * rhs.im) / (rhs.re * rhs.re + rhs.im + rhs.im),
            im: (self.im * rhs.re + self.re * rhs.im) / (rhs.re * rhs.re + rhs.im + rhs.im),
        }
    }
}

impl Div<f64> for Complex {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Self {
            re: self.re / rhs,
            im: self.im / rhs,
        }
    }
}

impl Neg for Complex {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            re: -self.re,
            im: -self.im,
        }
    }
}
