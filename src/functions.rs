use crate::{comp, complex::Complex};

pub fn lg(a: Option<&str>) -> Result<Complex, String> {
    if a.is_none() {
        return Err("Error.Need_More_Arguments".to_string());
    }
    let a = a.unwrap();
    if a.parse::<f64>().is_err() {
        return Err("Error.Invalid_Argument".to_string());
    }
    Ok(comp!(f64::log2(a.parse().unwrap()), 0.0))
}

pub fn ln(a: Option<&str>) -> Result<Complex, String> {
    if a.is_none() {
        return Err("Error.Need_More_Arguments".to_string());
    }
    let a = a.unwrap();
    if a.parse::<f64>().is_err() {
        return Err("Error.Invalid_Argument".to_string());
    }
    Ok(comp!(f64::ln(a.parse().unwrap()), 0.0))
}

pub fn sqrt(a: Option<&str>) -> Result<Complex, String> {
    if a.is_none() {
        return Err("Error.Need_More_Arguments".to_string());
    }
    let a = a.unwrap();
    if a.parse::<f64>().is_err() {
        return Err("Error.Invalid_Argument".to_string());
    }
    let k: f64 = a.parse().unwrap();
    Ok(Complex::sqrt(comp!(k)))
}

pub fn cbrt(a: Option<&str>) -> Result<Complex, String> {
    if a.is_none() {
        return Err("Error.Need_More_Arguments".to_string());
    }
    let a = a.unwrap();
    if a.parse::<f64>().is_err() {
        return Err("Error.Invalid_Argument".to_string());
    }
    Ok(comp!(f64::cbrt(a.parse().unwrap()), 0.0))
}
