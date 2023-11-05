use crate::{comp, complex::Complex};

pub fn lg(a: Option<&str>) -> Result<Complex, String> {
    if a == None {
        return Err("Error.Need_More_Arguments".to_string());
    }
    let a = a.unwrap();
    if a.parse::<f64>().is_err() {
        return Err("Error.Invalid_Argument".to_string());
    }
    return Ok(comp!(f64::log2(a.parse().unwrap()), 0.0));
}

pub fn ln(a: Option<&str>) -> Result<Complex, String> {
    if a == None {
        return Err("Error.Need_More_Arguments".to_string());
    }
    let a = a.unwrap();
    if a.parse::<f64>().is_err() {
        return Err("Error.Invalid_Argument".to_string());
    }
    return Ok(comp!(f64::ln(a.parse().unwrap()), 0.0));
}

pub fn sqrt(a: Option<&str>) -> Result<Complex, String> {
    if a == None {
        return Err("Error.Need_More_Arguments".to_string());
    }
    let a = a.unwrap();
    if a.parse::<f64>().is_err() {
        return Err("Error.Invalid_Argument".to_string());
    }
    let mut k: f64 = a.parse().unwrap();
    let is_ima = k < 0.0;
    if is_ima {
        k = -k;
    }
    let sqrt_result: f64 = f64::sqrt(k);
    if is_ima {
        return Ok(comp!(0.0, sqrt_result));
    }
    return Ok(comp!(sqrt_result, 0.0));
}

pub fn cbrt(a: Option<&str>) -> Result<Complex, String> {
    if a == None {
        return Err("Error.Need_More_Arguments".to_string());
    }
    let a = a.unwrap();
    if a.parse::<f64>().is_err() {
        return Err("Error.Invalid_Argument".to_string());
    }
    return Ok(comp!(f64::cbrt(a.parse().unwrap()), 0.0));
}
