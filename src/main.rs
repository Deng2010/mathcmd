use std::io::{self, Write};

mod solvex;

fn main() {
    let __version = String::from("0.1.0");
    println!("MATHcmd v{}", __version);
    loop {
        print!("mathcmd> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("ERROR: Unknown command!");
        let mut input_split = input.split_whitespace();
        let command = input_split.next().unwrap();
        if command == "+" {
            let a: f64 = input_split.next().unwrap().parse().unwrap();
            let b: f64 = input_split.next().unwrap().parse().unwrap();
            println!("{}", a + b);
        } else if command == "-" {
            let a: f64 = input_split.next().unwrap().parse().unwrap();
            let b: f64 = input_split.next().unwrap().parse().unwrap();
            println!("{}", a - b);
        } else if command == "*" {
            let a: f64 = input_split.next().unwrap().parse().unwrap();
            let b: f64 = input_split.next().unwrap().parse().unwrap();
            println!("{}", a * b);
        } else if command == "/" {
            let a: f64 = input_split.next().unwrap().parse().unwrap();
            let b: f64 = input_split.next().unwrap().parse().unwrap();
            println!("{}", a / b);
        } else if command == "solvex" {
            solvex::solvex::solvex_cmd();
        } else if command == "exit" {
            return;
        } else {
            println!("Error: {} is undefined!", command);
        }
    }
}
