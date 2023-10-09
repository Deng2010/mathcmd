pub fn calculator<'a>(a: f64, b: Option<&'a str>, sym: Option<&'a str>) -> Result<f64, String> {
    if sym.is_none() {
        return Ok(a);
    }
    if b.is_none(){
        return Err("Error.Need_More_Arguments".to_string());
    }
    let _first = a;
    let b = b.unwrap().parse::<f64>();
    if b.is_err() {
        return Err("Error.Invalid_Argument".to_string());
    }
    let _second = b.unwrap();
    let sym = sym.unwrap();
    match sym {
        "+" => Ok(_first + _second),
        "-" => Ok(_first - _second),
        "*" => Ok(_first * _second),
        "/" => Ok(_first / _second),
        "//" => Ok((_first as i64 / _second as i64) as f64),
        "%" => Ok(_first % _second),
        "^" | "**" => Ok(f64::powf(_first, _second)),
        "log" => Ok(f64::log(_first, _second)),
        _default => Err("Error.Unsupported_Operator".to_string())
    }
}
