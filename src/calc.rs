use rust_i18n::t;

pub fn calculator(a: f64, b: f64, sym: &str) -> f64 {
    match sym {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        "/" => a / b,
        "%" => a % b,
        "^" | "**" => f64::powf(a, b),
        "log" => f64::log(a, b),
        _default => {
            println!("{}", t!("Error.Unsupported_operator"));
            0.0
        }
    }
}
