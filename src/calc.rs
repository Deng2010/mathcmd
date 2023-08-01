pub mod calc {
    pub fn calc(a: f64, b: f64, sym: char) {
        // let a: f64 = command.parse().unwrap();
        // let sym: char = input_split.next().unwrap().parse().unwrap();
        // let b: f64 = input_split.next().unwrap().parse().unwrap();
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
