mod calc;
mod solvex;

use colored::*;
use std::io::{self, Write};
use std::str::SplitWhitespace;

fn main() {
    let lg: fn(f64) -> f64 = f64::log2;
    let ln: fn(f64) -> f64 = f64::ln;
    let calculator: fn(f64, f64, &str) -> f64 = calc::calculator;
    let solvex: fn() = solvex::solvex_mode;


    let __version = String::from(r#"0.1.2"#);
    let mut _cache: f64 = 0.0;


    loop {
        println!("{}", "mathcmd".bright_green());
        print!("{} ", ">".bright_cyan());
        io::stdout().flush().unwrap();


        let mut _input = String::new();
        io::stdin()
            .read_line(&mut _input)
            .expect("ERROR: Unknown command!");
        let mut input: SplitWhitespace = _input.split_whitespace();
        let __command = input.next();
        if __command.is_none() {
            continue;
        }
        let command: &str = __command.unwrap();

        if command.parse::<f64>().is_ok() { // command is digit
            let a: f64 = command.parse().unwrap();
            let sym: &str = input.next().unwrap();
            let b: f64 = input.next().unwrap().parse().unwrap();
            _cache = calculator(a, b, sym);
            println!("{}", _cache);
            continue;
        }
        match command { // else
            "+" | "-" | "*" | "/" | "^" | "%" | "log" => {
                let b: f64 = input.next().unwrap().parse().unwrap();
                _cache = calculator(_cache, b, command);
                println!("{}", _cache);
            },
            "solvex" => solvex(),
            "lg" => {
                let a: f64 = input.next().unwrap().parse().unwrap();
                println!("{}", lg(a));
            }
            "ln" => {
                let a: f64 = input.next().unwrap().parse().unwrap();
                println!("{}", ln(a));
            }
            "exit" | "ex" => return,
            "version" | "ver" | "v" => println!("MATHcmd v{}", __version),
            _default => println!("ERROR: Unknown command!"),
        }
    }
}
