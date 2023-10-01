
use crate::{
    calc::calculator,
    output::*,
    solve::solve_function_one_one,
};
use std::io;
use rust_i18n::*;
pub fn mathcmd_main() {
    let mut _input = String::new();
    let lg: fn(f64) -> f64 = f64::log2;
    let ln: fn(f64) -> f64 = f64::ln;
    let mut _version: String = String::from("0.2.1");
    let mut _cache: Result<f64, &str> = Ok(0.0);
    let mut _digit: f64 = 0.0;
    let mut _mem: f64 = 0.0;
    loop {
        command_prompt("mathcmd".to_string());
        let mut _input = String::new();
        io::stdin().read_line(&mut _input).unwrap();
        let mut input = _input.split_whitespace();
        let mut command = input.next().unwrap_or("");
        if command.parse::<f64>().is_ok() {
            _digit = command.parse().unwrap();
            command = "digit";
        }
        match command {
            "digit" => {
                let a: f64 = _digit;
                let sym: &str = input.next().unwrap();
                let b: f64 = input.next().unwrap().parse().unwrap();
                _cache = calculator(a, b, sym);
                output_cache(_cache, &mut _digit);
            }
            "+" | "-" | "*" | "/" | "//" | "^" | "%" | "log" => {
                let b: f64 = input.next().unwrap().parse().unwrap();
                _cache = calculator(_digit, b, command);
                output_cache(_cache, &mut _digit);
            }
            "solve" => solve_function_one_one(),
            "lg" => {
                let a: f64 = input.next().unwrap().parse().unwrap();
                _cache = Ok(lg(a));
                output_cache(_cache, &mut _digit);
            }
            "ln" => {
                let a: f64 = input.next().unwrap().parse().unwrap();
                _cache = Ok(ln(a));
                output_cache(_cache, &mut _digit);
            }
            "m+" => _mem += _digit,
            "m-" => _mem -= _digit,
            "mr" => {
                _cache = Ok(_mem);
                output_cache(_cache, &mut _digit);
            }
            "mc" => _mem = 0.0,
            "mrc" => {
                _cache = Ok(_mem);
                output_cache(_cache, &mut _digit);
                _mem = 0.0;
            }
            "exit" | "ex" => break,
            "version" | "ver" | "v" => output_ver(&mut _version),
            "" => (),
            "help" | "h" => {
                println!("{}", t!("Help"));
            }
            _ => {
                output_error("Error.Unknown_command");
            }
        }
    };
}
