pub mod calc {
    pub fn calc(a: f64, b: f64, sym: &str) {
        match sym {
            "+" => println!("{}", a + b),
            "-" => println!("{}", a - b),
            "*" => println!("{}", a * b),
            "/" => println!("{}", a / b),
            "%" => println!("{}", a % b),
            "^" => println!("{}", f64::powf(a, b)),
            "log" => println!("{}", f64::log(a, b)),
            _default => println!("ERROR: Unsupported operation!"),
        }
    }
}
