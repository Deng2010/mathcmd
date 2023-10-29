pub fn lg(a: Option<&str>) -> Result<f64, String> {
    if a == None {
        return Err("Error.Need_More_Arguments".to_string());
    }
    let a = a.unwrap();
    if a.parse::<f64>().is_err() {
        return Err("Error.Invalid_Argument".to_string());
    }
    return Ok(f64::log2(a.parse().unwrap()));
}
pub fn ln(a: Option<&str>) -> Result<f64, String> {
    if a == None {
        return Err("Error.Need_More_Arguments".to_string());
    }
    let a = a.unwrap();
    if a.parse::<f64>().is_err() {
        return Err("Error.Invalid_Argument".to_string());
    }
    return Ok(f64::ln(a.parse().unwrap()));
}
pub fn sqrt(a: Option<&str>) -> Result<f64, String> {
    if a == None {
        return Err("Error.Need_More_Arguments".to_string());
    }
    let a = a.unwrap();
    if a.parse::<f64>().is_err() {
        return Err("Error.Invalid_Argument".to_string());
    }
    return Ok(f64::sqrt(a.parse().unwrap()));
}
pub fn cbrt(a: Option<&str>) -> Result<f64, String> {
    if a == None {
        return Err("Error.Need_More_Arguments".to_string());
    }
    let a = a.unwrap();
    if a.parse::<f64>().is_err() {
        return Err("Error.Invalid_Argument".to_string());
    }
    return Ok(f64::cbrt(a.parse().unwrap()));
}
