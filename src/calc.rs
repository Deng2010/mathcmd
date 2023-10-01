pub fn calculator(a: f64, b: f64, sym: &str) -> Result<f64, &str> {
    match sym {
        "+" => Ok(a + b),
        "-" => Ok(a - b),
        "*" => Ok(a * b),
        "/" => Ok(a / b),
        "//" => Ok((a as i64 / b as i64) as f64),
        "%" => Ok(a % b),
        "^" | "**" => Ok(f64::powf(a, b)),
        "log" => Ok(f64::log(a, b)),
        _default => {
            Err("Error.Unsupported_operator")
        }
    }
    
}
