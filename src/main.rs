mod calc;
mod mathcmd_lib;
mod solvex;

use colored::*;
use mathcmd_lib::core;
use std::io::{self, Write};
fn main() {
    let __version = String::from("0.1.2");
    println!("MATHcmd v{}", __version);
    loop {
        print!("{}{} ", "mathcmd".bright_green(), ">".bright_cyan());
        io::stdout().flush().unwrap();
        let mut _input = String::new();
        io::stdin()
            .read_line(&mut _input)
            .expect("ERROR: Unknown command!");
        let mut input = _input.split_whitespace();
        let command = input.next().expect("FATAL ERROR: No command inputed!");
        if core::is_num(command.to_string()) {
            let a: f64 = command.parse().unwrap();
            let sym: char = input.next().unwrap().parse().unwrap();
            let b: f64 = input.next().unwrap().parse().unwrap();
            calc::calc::calc(a, b, sym);
        } else if command == "solvex" {
            solvex::solvex::solvex_mode();
        } else if command == "exit" {
            return;
        } else {
            println!("ERROR: {} is undefined!", command);
        }
    }
}
