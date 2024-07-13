use crate::{libs::complex::Comp, libs::expr::FuncRes, libs::point::Point};
use colored::*;
use std::io::{self, BufWriter, Write};
const ERROR: char = 'e';
const WARNING: char = 'w';
const INFO: char = 'i';
const RESULT: char = 'r';

#[macro_export]
macro_rules! print_ver {
    () => {
        println!("mathcmd {}", env!("CARGO_PKG_VERSION"))
    };
}

#[macro_export]
macro_rules! print_help {
    () => {
        println!(
            "{}",
            t!((String::from("help.") + std::env::var("mathcmd_page").unwrap().as_str()).as_str())
        )
    };
}

fn colorize(msg: String, msg_type: char) -> ColoredString {
    match msg_type {
        ERROR => msg.bold().red(),
        WARNING => msg.bold().yellow(),
        RESULT => msg.bold().cyan(),
        _ => msg.bold().italic().bright_cyan(),
    }
}

fn colorize_t(msg: &str, msg_type: char) -> ColoredString {
    colorize(t!(msg).to_string(), msg_type)
}

pub fn cmd_prompt(current: &str) {
    let mut handle = BufWriter::new(io::stdout());
    write!(handle, "{}\n> ", current.bold().bright_green()).unwrap();
    handle.flush().unwrap();
}

pub fn print_msg(msg: &str) {
    let mut handle = BufWriter::new(io::stderr());
    if msg.starts_with("error") {
        writeln!(handle, "{}", colorize_t(msg, ERROR)).unwrap();
    } else if msg.starts_with("warning") {
        writeln!(handle, "{}", colorize_t(msg, WARNING)).unwrap();
    } else if msg != "none" {
        writeln!(handle, "{}", colorize_t(msg, INFO)).unwrap();
    }
}

pub fn print_res(res: Result<Comp, String>) {
    match res {
        Ok(x) => println!(
            "{} {}",
            colorize(String::from("="), RESULT),
            colorize(x.to_string(), RESULT)
        ),
        Err(e) => print_msg(e.as_str()),
    }
}

pub fn print_pt(pt: Point) {
    println!("{}", colorize(pt.to_string(), RESULT));
}

pub fn print_func_res(res: FuncRes) {
    println!(
        "{} {} {}",
        colorize(res.get_name(), RESULT),
        colorize(String::from("="), RESULT),
        colorize(res.get_result().to_string(), RESULT)
    );
}
