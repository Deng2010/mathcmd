use crate::{comp, err, libs::complex::Complex};

pub fn lg(a: Option<&str>) -> Result<Complex, String> {
    if let Some(a) = a {
        if let Ok(a) = a.parse::<f64>() {
            return Ok(comp!(f64::log2(a)));
        }
        err!("error.invalid_argument")
    } else {
        err!("error.need_more_arguments")
    }
}

pub fn ln(a: Option<&str>) -> Result<Complex, String> {
    if let Some(a) = a {
        if let Ok(a) = a.parse::<f64>() {
            return Ok(comp!(f64::ln(a)));
        }
        err!("error.invalid_argument")
    } else {
        err!("error.need_more_arguments")
    }
}

pub fn sqrt(a: Option<&str>) -> Result<Complex, String> {
    if let Some(a) = a {
        if let Ok(a) = a.parse::<Complex>() {
            return Ok(Complex::sqrt(a));
        }
        err!("error.invalid_argument")
    } else {
        err!("error.need_more_arguments")
    }
}

pub fn cbrt(a: Option<&str>) -> Result<Complex, String> {
    if let Some(a) = a {
        if let Ok(a) = a.parse::<f64>() {
            return Ok(comp!(f64::cbrt(a)));
        }
        err!("error.invalid_argument")
    } else {
        err!("error.need_more_arguments")
    }
}

pub fn heron_formula(a: Complex, b: Complex, c: Complex) -> Complex {
    let s: Complex = (a + b + c) / 2.0;
    Complex::sqrt(s * (s - a) * (s - b) * (s - c))
}
