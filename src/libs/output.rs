use crate::{libs::complex::Complex, libs::expression::FunctionResult, libs::point::Point};
use colored::*;
use std::io::{self, BufWriter, Write};
pub fn command_prompt(_current: &str) {
    let mut handle = BufWriter::new(io::stdout());
    writeln!(handle, "{}", _current.bold().bright_green()).unwrap();
    write!(handle, "{} ", ">".bright_cyan()).unwrap();
    handle.flush().unwrap();
}

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
pub fn print_message(_message: &str) {
    let mut handle = BufWriter::new(io::stderr());
    if _message.starts_with("error") {
        writeln!(handle, "{}", t!(_message).bold().red()).unwrap();
    } else if _message.starts_with("warning") {
        writeln!(handle, "{}", t!(_message).bold().yellow()).unwrap();
    } else if _message != "none" {
        writeln!(handle, "{}", t!(_message).bold().italic().bright_cyan()).unwrap();
    }
}
pub fn print_result(_result: Result<Complex, String>) {
    match _result {
        Ok(x) => println!("{} {}", "=".bold().cyan(), x.to_string().bold().cyan()),
        Err(e) => print_message(e.as_str()),
    }
}
pub fn print_point(_point: Point) {
    println!("{}", _point.to_string().bold().cyan());
}
pub fn print_function_result(_result: FunctionResult) {
    println!(
        "{} {} {}",
        _result.get_name().bold().cyan(),
        "=".bold().cyan(),
        _result.get_result().to_string().bold().cyan()
    );
}
