pub mod calc {
    pub fn calc(a: f64, b: f64, sym: char) {
        match sym {
            '+' => println!("{}", a + b),
            '-' => println!("{}", a - b),
            '*' => println!("{}", a * b),
            '/' => println!("{}", a / b),
            '^' => println!("{}", f64::powf(a, b)),
            _default => println!("ERROR: Unsupported operation!"),
        }
    }
}
