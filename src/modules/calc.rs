//Current page: calc

use std::{
    f64::consts::{PI, TAU},
    io::stdin,
    str::SplitWhitespace,
};

use crate::{
    comp, err,
    functions::*,
    libs::{
        complex::Complex,
        memory::Memory,
        output::{command_prompt, output_help, output_result},
    },
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
        err!("error.need_more_arguments")
    }
    let rhs = rhs.unwrap().to_owned().parse();
    if rhs.is_err() {
        err!("error.invalid_argument")
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

const PAGE: &str = "calc";

pub fn calc_main() {
    let mut cache: Complex = comp!();
    let mut _mem: Memory = Memory::new();
    loop {
        command_prompt("mathcmd->calc");
        let mut input: String = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut input: SplitWhitespace = input.split_whitespace();
        if input.clone().count() == 0 {
            continue;
        }
        let mut command: &str = input.next().unwrap();
        if command.parse::<Complex>().is_ok() {
            cache = command.parse().unwrap();
            command = "digit";
        }
        let nxt: Option<&str>;
        let operators: [&str; 9] = ["+", "-", "*", "**", "/", "//", "^", "%", "log"];
        let op: Option<&str> = match operators.contains(&command) {
            true => Some(command),
            false => None,
        };
        if op.is_some() {
            command = "operator";
        }
        let _cache: Result<Complex, String> = match command {
            "digit" => {
                let op: Option<&str> = input.next();
                nxt = input.next();
                calculator(cache, nxt, op)
            }
            "operator" => {
                nxt = input.next();
                calculator(cache, nxt, op)
            }
            "solve" => {
                solve();
                Err(String::from("none"))
            }
            "lg" => lg(input.next()),
            "ln" => ln(input.next()),
            "sqrt" => sqrt(input.next()),
            "cbrt" => cbrt(input.next()),
            "pi" => Ok(comp!(PI)),
            "tau" => Ok(comp!(TAU)),
            "e" => Ok(comp!(f64::exp(1.0))),
            "m+" => _mem.add(cache),
            "m-" => _mem.sub(cache),
            "mr" => Ok(_mem.get()),
            "mc" => _mem.reset(),
            "mrc" => _mem.get_reset(),
            "exit" | "ex" => break,
            "help" | "h" => {
                output_help(PAGE);
                Err(String::from("none"))
            }
            _ => Err(String::from("error.unknown_command")),
        };
        if _cache.is_ok() {
            cache = _cache.clone().unwrap();
        }
        output_result(_cache);
    }
}
