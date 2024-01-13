//Current page: calc

use std::{
    f64::consts::{PI, TAU},
    io::stdin,
};

use crate::{
    comp,
    functions::*,
    libs::cache::Cache,
    libs::complex::Complex,
    libs::memory::Memory,
    libs::output::{command_prompt, output_help, output_message, output_ver},
    modules::solve::solve_main as solve,
};

pub fn calculator<'a>(
    lhs: Complex,
    rhs: Option<&'a str>,
    op: Option<&'a str>,
) -> Result<Complex, String> {
    if op.is_none() {
        return Ok(lhs);
    }
    if rhs.is_none() {
        return Err("error.need_more_arguments".to_string());
    }
    let rhs = rhs.unwrap().to_owned().parse();
    if rhs.is_err() {
        return Err("error.invalid_argument".to_string());
    }
    let rhs: Complex = rhs.unwrap();
    let op = op.unwrap();
    match op {
        "+" => Ok(lhs + rhs),
        "-" => Ok(lhs - rhs),
        "*" => Ok(lhs * rhs),
        "/" => Ok(lhs / rhs),
        "//" => Ok(comp!(
            (lhs.to_num() as i64 / rhs.to_num() as i64) as f64,
            0.0
        )),
        "%" => Ok(comp!(lhs.to_num() % rhs.to_num(), 0.0)),
        "^" | "**" => Ok(Complex::pow(lhs, rhs.to_num() as u32)),
        "log" => Ok(comp!(f64::log(lhs.to_num(), rhs.to_num()), 0.0)),
        _ => Err("error.unsupported_operator".to_string()),
    }
}
pub fn calc_main() {
    let page: &str = "calc";
    let mut _cache: Cache = Cache::new();
    let mut command: &str;
    let mut _mem: Memory = Memory::new();
    loop {
        command_prompt("mathcmd->calc");
        let mut _input: String = String::new();
        stdin().read_line(&mut _input).unwrap();
        let mut input = _input.split_whitespace();
        let opt_command: Option<&str> = input.next();
        if opt_command.is_none() {
            continue;
        }
        command = opt_command.unwrap();
        if command.parse::<Complex>().is_ok() {
            _cache.update_digit(command.parse().unwrap());
            command = "digit";
        }
        let mut op: Option<&str> = None;
        let nxt: Option<&str>;
        let operators: [&str; 9] = ["+", "-", "*", "**", "/", "//", "^", "%", "log"];
        if operators.contains(&command) {
            op = Some(command);
            command = "operator";
        }
        match command {
            "digit" => {
                op = input.next();
                nxt = input.next();
                _cache.update_output(calculator(_cache.get_digit(), nxt, op));
            }
            "operator" => {
                nxt = input.next();
                _cache.update_output(calculator(_cache.get_digit(), nxt, op));
            }
            "solve" => solve(),
            "lg" => {
                nxt = input.next();
                _cache.update_output(lg(nxt));
            }
            "ln" => {
                nxt = input.next();
                _cache.update_output(ln(nxt));
            }
            "sqrt" => {
                nxt = input.next();
                _cache.update_output(sqrt(nxt));
            }
            "cbrt" => {
                nxt = input.next();
                _cache.update_output(cbrt(nxt));
            }
            "pi" => {
                _cache.update_output(Ok(comp!(PI)));
            }
            "tau" => {
                _cache.update_output(Ok(comp!(TAU)));
            }
            "m+" => _mem += _cache.get_digit(),
            "m-" => _mem -= _cache.get_digit(),
            "mr" => {
                _cache.update_output(Ok(_mem.get()));
            }
            "mc" => _mem.reset(),
            "mrc" => {
                _cache.update_output(Ok(_mem.get()));
                _mem.reset();
            }
            "exit" | "ex" => break,
            "version" | "ver" | "v" => output_ver(),
            "help" | "h" => output_help(page),
            _ => output_message("error.unknown_command"),
        }
    }
}
