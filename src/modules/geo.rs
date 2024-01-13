//Current page: geo

use std::{collections::HashMap, io::stdin};

use crate::libs::{
    cache::Cache,
    complex::Complex,
    memory::Memory,
    output::{command_prompt, output_help, output_message, output_ver},
    point::{Line, Point},
};

pub fn geo_main() {
    let page: &str = "geo";
    let mut points: HashMap<String, Point> = HashMap::new();
    let mut lines: HashMap<String, Line> = HashMap::new();
    let mut _cache: Cache = Cache::new();
    let mut command: &str;
    let mut _mem: Memory = Memory::new();
    loop {
        command_prompt("mathcmd->geo");
        let mut _input: String = String::new();
        stdin().read_line(&mut _input).unwrap();
        let mut input = _input.split_whitespace();
        let opt_command: Option<&str> = input.next();
        if opt_command.is_none() {
            continue;
        }
        command = opt_command.unwrap();
        match command {
            "point" => {
                let name: Option<&str> = input.next();
                let x: Option<&str> = input.next();
                let y: Option<&str> = input.next();
                if y.is_none() {
                    _cache.update_output(Err("error.need_more_arguments".to_string()));
                    continue;
                } else if x.unwrap().parse::<Complex>().is_err()
                    || y.unwrap().parse::<Complex>().is_err()
                {
                    _cache.update_output(Err("error.invalid_argument".to_string()));
                    continue;
                }
                let name: &str = name.unwrap();
                let x: Complex = x.unwrap().parse().unwrap();
                let y: Complex = y.unwrap().parse().unwrap();
                points.insert(name.to_owned(), Point::new(x, y));
            }
            "line" => {
                let name: Option<&str> = input.next();
                let xname: Option<&str> = input.next();
                let yname: Option<&str> = input.next();
                if yname.is_none() {
                    _cache.update_output(Err("error.need_more_arguments".to_string()));
                    continue;
                }
                let name: &str = name.unwrap();
                let x: Option<&Point> = points.get(xname.unwrap());
                let y: Option<&Point> = points.get(yname.unwrap());
                if x.is_none() || y.is_none() {
                    _cache.update_output(Err("error.invalid_argument".to_string()));
                    continue;
                }
                lines.insert(name.to_owned(), Line::new(*x.unwrap(), *y.unwrap()));
            }
            "dis" => {
                let name: Option<&str> = input.next();
                if name.is_none() {
                    _cache.update_output(Err("error.need_more_arguments".to_string()));
                    continue;
                }
                let name: &str = name.unwrap();
                let line: Option<&Line> = lines.get(name);
                if line.is_none() {
                    _cache.update_output(Err("error.invalid_argument".to_string()));
                    continue;
                }
                _cache.update_output(Ok(line.unwrap().dis()));
            }
            "version" | "ver" | "v" => output_ver(),
            "exit" | "ex" => break,
            "help" | "h" => output_help(page),
            "" => continue,
            _ => output_message("error.unknown_command"),
        }
    }
}
