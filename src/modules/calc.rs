//Current page: calc

use std::{
    f64::consts::{PI, TAU},
    io::stdin,
};

use crate::{
    cache::Cache,
    comp,
    complex::Complex,
    functions::*,
    memory::Memory,
    modules::solve::solve_main as solve,
    operators,
    output::{command_prompt, output_help, output_message, output_ver},
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
        command = input.next().unwrap_or("");
        if command.parse::<Complex>().is_ok() {
            _cache.update_digit(command.parse().unwrap());
            command = "digit";
        }
        let mut _op: Option<&str> = None;
        let mut _nxt: Option<&str> = None;
        let operators = operators!();
        if operators.contains(&command) {
            _op = Some(command);
            command = "operator";
        }
        match command {
            "digit" => {
                _op = input.next();
                _nxt = input.next();
                _cache.update(calculator(_cache.get_digit(), _nxt, _op));
                _cache.output();
            }
            "operator" => {
                _nxt = input.next();
                _cache.update(calculator(_cache.get_digit(), _nxt, _op));
                _cache.output();
            }
            "solve" => solve(),
            "lg" => {
                _nxt = input.next();
                _cache.update(lg(_nxt));
                _cache.output();
            }
            "ln" => {
                _nxt = input.next();
                _cache.update(ln(_nxt));
                _cache.output();
            }
            "sqrt" => {
                _nxt = input.next();
                _cache.update(sqrt(_nxt));
                _cache.output();
            }
            "cbrt" => {
                _nxt = input.next();
                _cache.update(cbrt(_nxt));
                _cache.output();
            }
            "pi" => {
                _cache.update(Ok(comp!(PI)));
                _cache.output();
            }
            "tau" => {
                _cache.update(Ok(comp!(TAU)));
                _cache.output();
            }
            "m+" => _mem += _cache.get_digit(),
            "m-" => _mem -= _cache.get_digit(),
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
            "help" | "h" => output_help(page),
            _ => output_message("error.unknown_command"),
        }
    }
}
