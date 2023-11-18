use crate::{comp, complex::Complex};

pub fn calculator<'a>(
    a: Complex,
    b: Option<&'a str>,
    sym: Option<&'a str>,
) -> Result<Complex, String> {
    if sym.is_none() {
        return Ok(a);
    }
    if b.is_none() {
        return Err("Error.Need_More_Arguments".to_string());
    }
    let mut _first = a;
    let b = b.unwrap().to_owned().parse();
    if b.is_err() {
        return Err("Error.Invalid_Argument".to_string());
    }
    let mut _second: Complex = b.unwrap();
    let sym = sym.unwrap();
    match sym {
        "+" => Ok(_first + _second),
        "-" => Ok(_first - _second),
        "*" => Ok(_first * _second),
        "/" => Ok(_first / _second),
        "//" => Ok(comp!(
            (_first.to_num() as i64 / _second.to_num() as i64) as f64,
            0.0
        )),
        "%" => Ok(comp!(_first.to_num() % _second.to_num(), 0.0)),
        "^" | "**" => Ok(Complex::pow(_first, _second.to_num() as u32)),
        "log" => Ok(comp!(f64::log(_first.to_num(), _second.to_num()), 0.0)),
        _ => Err("Error.Unsupported_Operator".to_string()),
    }
}
