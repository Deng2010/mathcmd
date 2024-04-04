use crate::{comp, err, libs::complex::Complex};

pub fn lg(a: Option<&str>) -> Result<Complex, String> {
    if a.is_none() {
        err!("error.need_more_arguments");
    }
    let a: &str = a.unwrap();
    if a.parse::<f64>().is_err() {
        err!("error.invalid_argument");
    }
    Ok(comp!(f64::log2(a.parse().unwrap()), 0.0))
}

pub fn ln(a: Option<&str>) -> Result<Complex, String> {
    if a.is_none() {
        err!("error.need_more_arguments");
    }
    let a: &str = a.unwrap();
    if a.parse::<f64>().is_err() {
        err!("error.invalid_argument");
    }
    Ok(comp!(f64::ln(a.parse().unwrap()), 0.0))
}

pub fn sqrt(a: Option<&str>) -> Result<Complex, String> {
    if a.is_none() {
        err!("error.need_more_arguments");
    }
    let a: &str = a.unwrap();
    if a.parse::<Complex>().is_err() {
        err!("error.invalid_argument");
    }
    Ok(Complex::sqrt(a.parse().unwrap()))
}

pub fn cbrt(a: Option<&str>) -> Result<Complex, String> {
    if a.is_none() {
        err!("error.need_more_arguments");
    }
    let a: &str = a.unwrap();
    if a.parse::<f64>().is_err() {
        err!("error.invalid_argument");
    }
    Ok(comp!(f64::cbrt(a.parse().unwrap()), 0.0))
}

pub fn heron_formula(a: Complex, b: Complex, c: Complex) -> Complex {
    let s: Complex = (a + b + c) / 2.0;
    Complex::sqrt(s * (s - a) * (s - b) * (s - c))
}
