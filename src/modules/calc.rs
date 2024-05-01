//Current page: calc

use std::{env, io, str::SplitWhitespace};

use crate::{
    comp, err,
    functions::*,
    libs::{
        complex::{
            consts::{CE, CPI, CTAU},
            Complex,
        },
        memory::Memory,
        output::{command_prompt, print_result},
    },
    print_help, print_ver,
};

pub fn calculator<'a>(
    lhs: Complex,
    rhs: Option<&'a str>,
    op: Option<&'a str>,
) -> Result<Complex, String> {
    if let Some(op) = op {
        if let Some(rhs) = rhs {
            if let Ok(rhs) = rhs.to_owned().parse::<Complex>() {
                match op {
                    "+" => Ok(lhs + rhs),
                    "-" => Ok(lhs - rhs),
                    "*" => Ok(lhs * rhs),
                    "/" => Ok(lhs / rhs),
                    "//" => Ok(comp!((lhs / rhs).to_num().floor())),
                    "%" => Ok(comp!(lhs.to_num() % rhs.to_num())),
                    "^" | "**" => Ok(Complex::pow(lhs, rhs.to_num() as u32)),
                    "log" => Ok(comp!(f64::log(lhs.to_num(), rhs.to_num()))),
                    _ => err!("error.unsupported_operator"),
                }
            } else {
                err!("error.invalid_argument")
            }
        } else {
            err!("error.need_more_arguments")
        }
    } else {
        Ok(lhs)
    }
}

static OPERATOR_MAP: &[&str; 9] = &["+", "-", "*", "**", "/", "//", "^", "%", "log"];

pub fn calc_main() {
    env::set_var("mathcmd_page", "calc");
    let mut cache: Complex = comp!();
    let mut mem: Memory = Memory::new();
    let mut input: SplitWhitespace;
    loop {
        command_prompt("mathcmd->calc");
        let mut reading: String = String::new();
        io::stdin()
            .read_line(&mut reading)
            .expect("Failed to read input");
        input = reading.split_whitespace();
        if input.clone().count() == 0 {
            continue;
        }
        let command: &str = input.next().unwrap();
        let (command, op, nxt): (&str, Option<&str>, Option<&str>) =
            if let Ok(lhs) = command.parse::<Complex>() {
                cache = lhs;
                ("expr", input.next(), input.next())
            } else if OPERATOR_MAP.contains(&command) {
                let tmp: &str = command;
                ("expr", Some(tmp), input.next())
            } else {
                (command, None, input.next())
            };
        let _cache: Result<Complex, String> = match command {
            "expr" => calculator(cache, nxt, op),
            "lg" => lg(nxt),
            "ln" => ln(nxt),
            "sqrt" => sqrt(nxt),
            "cbrt" => cbrt(nxt),
            "pi" => Ok(CPI),
            "tau" => Ok(CTAU),
            "e" => Ok(CE),
            "m+" => mem.add(cache),
            "m-" => mem.sub(cache),
            "mr" => Ok(mem.get()),
            "mc" => mem.reset(),
            "mrc" => mem.get_reset(),
            "exit" | "ex" => break,
            "version" | "ver" | "v" => {
                print_ver!();
                err!("none")
            }
            "help" | "h" => {
                print_help!();
                Err(String::from("none"))
            }
            _ => Err(String::from("error.unknown_command")),
        };
        if let Ok(x) = _cache {
            cache = x;
        }
        print_result(_cache);
    }
}
