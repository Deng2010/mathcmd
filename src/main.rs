mod calc;
mod solvex;

use colored::*;
use std::io::{self, Write};
fn main() {
    let __version = String::from(r#"0.1.2"#);
    loop {
        println!("{}", "mathcmd".bright_green());
        print!("{} ", ">".bright_cyan());
        io::stdout().flush().unwrap();
        let mut _input = String::new();
        io::stdin()
            .read_line(&mut _input)
            .expect("ERROR: Unknown command!");
        let mut input = _input.split_whitespace();
        let __command = input.next();
        if __command.is_none() {
            continue;
        }
        let command = __command.unwrap();
        if command.parse::<f64>().is_ok() {
            let a: f64 = command.parse().unwrap();
            let sym = input.next().unwrap();
            let b: f64 = input.next().unwrap().parse().unwrap();
            calc::calc::calc(a, b, sym);
            continue;
        }
        match command {
            "solvex" => solvex::solvex::solvex_mode(),
            "exit" | "ex" => return,
            "version" | "ver" | "v" => println!("MATHcmd v{}", __version),
            _default => println!("ERROR: Unknown command!"),
        }
    }
}
