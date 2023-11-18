use crate::{
    cache::Cache,
    comp,
    complex::Complex,
    functions::*,
    memory::Memory,
    modules::{calc::calculator, solve::solve_function},
    operators,
    output::{command_prompt, output_help, output_message, output_ver},
};
use std::{f64::consts::PI, io::stdin};
pub fn mathcmd_main() {
    let mut _cache: Cache = Cache::new(Ok(comp!(0.0, 0.0)));
    let mut command: &str;
    let mut _mem: Memory = Memory::new();
    loop {
        command_prompt("mathcmd");
        let mut _input: String = String::new();
        stdin().read_line(&mut _input).unwrap();
        let mut input = _input.split_whitespace();
        command = input.next().unwrap_or("");
        if command.parse::<Complex>().is_ok() {
            _cache.update_digit(command.parse().unwrap());
            command = "digit";
        }
        let mut _operator: Option<&str> = None;
        let operators = operators!();
        if operators.contains(&command) {
            _operator = Some(command);
            command = "operator";
        }
        match command {
            "digit" => {
                _operator = input.next();
                let nxt: Option<&str> = input.next();
                _cache.update(calculator(_cache.get_digit(), nxt, _operator));
                _cache.output();
            }
            "operator" => {
                let nxt: Option<&str> = input.next();
                _cache.update(calculator(_cache.get_digit(), nxt, _operator));
                _cache.output();
            }
            "solve" => solve_function(),
            "lg" => {
                let nxt: Option<&str> = input.next();
                _cache.update(lg(nxt));
                _cache.output();
            }
            "ln" => {
                let nxt: Option<&str> = input.next();
                _cache.update(ln(nxt));
                _cache.output();
            }
            "sqrt" => {
                let nxt: Option<&str> = input.next();
                _cache.update(sqrt(nxt));
                _cache.output();
            }
            "cbrt" => {
                let nxt: Option<&str> = input.next();
                _cache.update(cbrt(nxt));
                _cache.output();
            }
            "pi" => {
                _cache.update(Ok(comp!(PI, 0.0)));
                _cache.output();
            }
            "m+" => _mem.add(_cache.get_digit()),
            "m-" => _mem.add(-_cache.get_digit()),
            "mr" => {
                _cache.update(Ok(_mem.get()));
                _cache.output();
            }
            "mc" => _mem.reset(),
            "mrc" => {
                _cache.update(Ok(_mem.get()));
                _cache.output();
                _mem.reset();
            }
            "exit" | "ex" => break,
            "version" | "ver" | "v" => output_ver(),
            "" => (),
            "help" | "h" => output_help("main"),
            _ => output_message("Error.Unknown_Command"),
        }
    }
}
