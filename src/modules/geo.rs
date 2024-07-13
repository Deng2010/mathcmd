//Current page: geo

use std::{collections::HashMap, env, io::stdin, str::SplitWhitespace};

use crate::{
    functions::geometry::heron_formula,
    libs::{
        complex::Comp,
        output::{cmd_prompt, print_res},
        point::{Line, Point},
    },
    print_help,
};

pub fn geo_main() {
    env::set_var("mathcmd_page", "geo");
    let mut points: HashMap<String, Point> = HashMap::new();
    let mut lines: HashMap<String, Line> = HashMap::new();
    let mut input: SplitWhitespace;
    loop {
        cmd_prompt("mathcmd->geo");
        let mut _input: String = String::new();
        stdin().read_line(&mut _input).unwrap();
        input = _input.split_whitespace();
        if input.clone().count() == 0 {
            continue;
        }
        let cache: Result<Comp, String> = match input.next().unwrap() {
            "point" => match (input.next(), input.next(), input.next()) {
                (Some(name), Some(x), Some(y)) => match (x.parse::<Comp>(), y.parse::<Comp>()) {
                    (Ok(x), Ok(y)) => {
                        points.insert(name.to_owned(), Point::new(x, y));
                        Err(String::from("none"))
                    }
                    _ => Err("error.invalid_argument".to_string()),
                },
                _ => Err("error.need_more_arguments".to_string()),
            },
            "line" => match (input.next(), input.next(), input.next()) {
                (Some(name), Some(x), Some(y)) => {
                    match Line::new_option(points.get(x), points.get(y)) {
                        Ok(x) => {
                            lines.insert(name.to_owned(), x);
                            Err(String::from("none"))
                        }
                        Err(e) => Err(e),
                    }
                }
                _ => Err("error.need_more_arguments".to_string()),
            },
            "dis" => match input.next() {
                Some(name) => match lines.get(name) {
                    Some(x) => Ok(x.dis()),
                    None => Err("error.invalid_argument".to_string()),
                },
                None => Err("error.need_more_arguments".to_string()),
            },
            "triarea" => match (input.next(), input.next(), input.next()) {
                (Some(a), Some(b), Some(c)) => match (lines.get(a), lines.get(b), lines.get(c)) {
                    (Some(a), Some(b), Some(c)) => Ok(heron_formula(a.dis(), b.dis(), c.dis())),
                    _ => Err("error.invalid_argument".to_string()),
                },
                _ => Err("error.need_more_arguments".to_string()),
            },
            "exit" | "ex" => break,
            "help" | "h" => {
                print_help!();
                Err(String::from("none"))
            }
            _ => Err(String::from("error.unknown_command")),
        };
        print_res(cache);
    }
}
