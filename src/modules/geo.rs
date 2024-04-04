//Current page: geo

use std::{collections::HashMap, io::stdin};

use crate::{
    functions::heron_formula,
    libs::{
        complex::Complex,
        memory::Memory,
        output::{command_prompt, output_help, output_result},
        point::{Line, Point},
    },
};

const PAGE: &str = "geo";

pub fn geo_main() {
    let mut points: HashMap<String, Point> = HashMap::new();
    let mut lines: HashMap<String, Line> = HashMap::new();
    let mut command: &str;
    let mut _mem: Memory = Memory::new();
    loop {
        command_prompt("mathcmd->geo");
        let mut input: String = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut input = input.split_whitespace();
        if input.clone().count() == 0 {
            continue;
        }
        let opt_command: Option<&str> = input.next();
        if opt_command.is_none() {
            continue;
        }
        command = opt_command.unwrap();
        let cache: Result<Complex, String> = match command {
            "point" => {
                let (name, x, y): (Option<&str>, Option<&str>, Option<&str>) =
                    (input.next(), input.next(), input.next());
                let _point: Result<Point, String> = Point::new_option(x, y);
                if let Err(e) = _point {
                    Err(e)
                } else {
                    let name = name.unwrap();
                    points.insert(name.to_owned(), _point.unwrap());
                    Err(String::from("none"))
                }
            }
            "line" => {
                let (name, xname, yname): (Option<&str>, Option<&str>, Option<&str>) =
                    (input.next(), input.next(), input.next());
                if let (Some(x), Some(y)) = (xname, yname) {
                    let name: &str = name.unwrap();
                    let _line: Result<Line, String> =
                        Line::new_option(points.get(x), points.get(y));
                    if let Err(e) = _line {
                        Err(e)
                    } else {
                        lines.insert(name.to_owned(), _line.unwrap());
                        Err(String::from("none"))
                    }
                } else {
                    Err("error.need_more_arguments".to_string())
                }
            }
            "dis" => {
                let name: Option<&str> = input.next();
                if let Some(name) = name {
                    if let Some(line) = lines.get(name) {
                        Ok(line.dis())
                    } else {
                        Err("error.invalid_argument".to_string())
                    }
                } else {
                    Err("error.need_more_arguments".to_string())
                }
            }
            "triarea" => {
                let (a, b, c): (Option<&str>, Option<&str>, Option<&str>) =
                    (input.next(), input.next(), input.next());
                if let (Some(a), Some(b), Some(c)) = (a, b, c) {
                    let (a, b, c): (Option<&Line>, Option<&Line>, Option<&Line>) =
                        (lines.get(a), lines.get(b), lines.get(c));
                    if let (Some(a), Some(b), Some(c)) = (a, b, c) {
                        Ok(heron_formula(a.dis(), b.dis(), c.dis()))
                    } else {
                        Err("error.invalid_argument".to_string())
                    }
                } else {
                    Err("error.need_more_arguments".to_string())
                }
            }
            "exit" | "ex" => break,
            "help" | "h" => {
                output_help(PAGE);
                Err(String::from("none"))
            }
            _ => Err(String::from("error.unknown_command")),
        };
        output_result(cache);
    }
}
