pub fn calculator(a: f64, b: f64, sym: &str) -> f64 {
    match sym {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        "/" => a / b,
        "%" => a % b,
        "^" => f64::powf(a, b),
        "log" => f64::log(a, b),
        _default => {
            println!("ERROR: Unsupported operation!");
            0.0
        }
    }
}

