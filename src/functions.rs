use crate::{comp, complex::Complex};

pub fn lg(a: Option<&str>) -> Result<Complex, String> {
    if a.is_none() {
        return Err("error.need_more_arguments".to_string());
    }
    let a = a.unwrap();
    if a.parse::<f64>().is_err() {
        return Err("error.invalid_argument".to_string());
    }
    Ok(comp!(f64::log2(a.parse().unwrap()), 0.0))
}

pub fn ln(a: Option<&str>) -> Result<Complex, String> {
    if a.is_none() {
        return Err("error.need_more_arguments".to_string());
    }
    let a = a.unwrap();
    if a.parse::<f64>().is_err() {
        return Err("error.invalid_argument".to_string());
    }
    Ok(comp!(f64::ln(a.parse().unwrap()), 0.0))
}

pub fn sqrt(a: Option<&str>) -> Result<Complex, String> {
    if a.is_none() {
        return Err("error.need_more_arguments".to_string());
    }
    let a = a.unwrap();
    if a.parse::<Complex>().is_err() {
        return Err("error.invalid_argument".to_string());
    }
    Ok(Complex::sqrt(a.parse().unwrap()))
}

pub fn cbrt(a: Option<&str>) -> Result<Complex, String> {
    if a.is_none() {
        return Err("error.need_more_arguments".to_string());
    }
    let a = a.unwrap();
    if a.parse::<f64>().is_err() {
        return Err("error.invalid_argument".to_string());
    }
    Ok(comp!(f64::cbrt(a.parse().unwrap()), 0.0))
}
