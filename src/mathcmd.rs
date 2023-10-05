use crate::{
    calc::calculator,
    data::operators,
    functions::*,
    memory_calc::Memory,
    output::{command_prompt, output_cache, output_error, output_help, output_ver},
    solve::solve_function_one_one,
};
use std::{f64::consts::PI, io::stdin};
pub fn mathcmd_main() {
    let mut _cache: Result<f64, &str> = Err("Error.The_Cache_Is_Empty");
    let mut _digit: f64 = 0.0;
    let mut command: &str;
    let mut _mem: Memory = Memory::new();
    loop {
        command_prompt("mathcmd");
        let mut _input: String = String::new();
        stdin().read_line(&mut _input).unwrap();
        let mut input = _input.split_whitespace();
        command = input.next().unwrap_or("");
        if command.parse::<f64>().is_ok() {
            _digit = command.parse().unwrap();
            command = "digit";
        }
        let mut _operator: Option<&str> = None;
        let operators = operators();
        for operator in operators {
            if command == operator {
                _operator = Some(command);
                command = "operator";
            }
        }
        match command {
            "digit" => {
                _operator = input.next();
                let nxt: Option<&str> = input.next();
                _cache = calculator(_digit, nxt, _operator);
                output_cache(_cache, &mut _digit);
            }
            "operator" => {
                let nxt: Option<&str> = input.next();
                _cache = calculator(_digit, nxt, _operator);
                output_cache(_cache, &mut _digit);
            }
            "solve" => solve_function_one_one(),
            "lg" => {
                let nxt: Option<&str> = input.next();
                _cache = lg(nxt);
                output_cache(_cache, &mut _digit);
            }
            "ln" => {
                let nxt: Option<&str> = input.next();
                _cache = ln(nxt);
                output_cache(_cache, &mut _digit);
            }
            "sqrt" => {
                let nxt: Option<&str> = input.next();
                _cache = sqrt(nxt);
                output_cache(_cache, &mut _digit);
            }
            "cbrt" => {
                let nxt: Option<&str> = input.next();
                _cache = cbrt(nxt);
                output_cache(_cache, &mut _digit);
            }
            "pi" => {
                _cache = Ok(PI);
                output_cache(_cache, &mut _digit);
            }
            "m+" => _mem.add(_digit),
            "m-" => _mem.add(-_digit),
            "mr" => {
                _cache = Ok(_mem.get());
                output_cache(_cache, &mut _digit);
            }
            "mc" => _mem.reset(),
            "mrc" => {
                _cache = Ok(_mem.get());
                output_cache(_cache, &mut _digit);
                _mem.reset();
            }
            "exit" | "ex" => break,
            "version" | "ver" | "v" => output_ver(),
            "" => (),
            "help" | "h" => output_help("main"),
            _ => output_error("Error.Unknown_Command"),
        }
    }
}
