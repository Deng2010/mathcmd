pub mod calc {
    pub fn calc(a: f64, b: f64, sym: char) {
        if sym == '+' {
            println!("{}", a + b);
        } else if sym == '-' {
            println!("{}", a - b);
        } else if sym == '*' {
            println!("{}", a * b);
        } else if sym == '/' {
            println!("{}", a / b);
        }
    }
}
