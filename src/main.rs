mod calc;
mod output;
mod solvex;

use crate::output::{command_prompt, output_cache, output_ver};
use std::io;
use std::str::SplitWhitespace;

fn main() {
    let lg: fn(f64) -> f64 = f64::log2;
    let ln: fn(f64) -> f64 = f64::ln;
    let calculator: fn(f64, f64, &str) -> f64 = calc::calculator;
    let solvex: fn() = solvex::solvex_mode;

    let mut _version: String = String::from("0.1.2");
    let mut _cache: f64 = 0.0;
    let mut _mem: f64 = 0.0;

    loop {
        command_prompt();

        let mut _input = String::new();
        io::stdin()
            .read_line(&mut _input)
            .expect("ERROR: Unexpected error!");
        let mut input: SplitWhitespace = _input.split_whitespace();
        let __command = input.next();
        if __command.is_none() {
            continue;
        }
        let command: &str = __command.unwrap();

        if command.parse::<f64>().is_ok() {
            // command is digit
            let a: f64 = command.parse().unwrap();
            let sym: &str = input.next().unwrap();
            let b: f64 = input.next().unwrap().parse().unwrap();
            _cache = calculator(a, b, sym);
            output_cache(_cache);
            continue;
        }
        match command {
            // else
            "+" | "-" | "*" | "/" | "^" | "%" | "log" => {
                let b: f64 = input.next().unwrap().parse().unwrap();
                _cache = calculator(_cache, b, command);
                output_cache(_cache);
            }
            "solvex" => solvex(),
            "lg" => {
                if input.next().is_some() {
                    let a: f64 = input.next().unwrap().parse().unwrap();
                    _cache = lg(a);
                }
                output_cache(_cache);
            }
            "ln" => {
                if input.next().is_some() {
                    let a: f64 = input.next().unwrap().parse().unwrap();
                    _cache = ln(a);
                }
                output_cache(_cache);
            }
            "m+" => _mem += _cache,
            "m-" => _mem -= _cache,
            "mr" => {
                _cache = _mem;
                output_cache(_cache);
            }
            "mc" => _mem = 0.0,
            "mrc" => {
                _mem += _cache;
                output_cache(_cache);
                _mem = 0.0;
            }
            "exit" | "ex" => return,
            "version" | "ver" | "v" => output_ver(&mut _version),
            _default => println!("ERROR: Unknown command!"),
        }
    }
}
