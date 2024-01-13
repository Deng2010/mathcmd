use crate::{
    libs::complex::Complex, libs::expression::FunctionResult, libs::point::Point, VERSION,
};
use colored::*;
use std::io::{self, Write};
pub fn command_prompt(_current: &str) {
    let mut handle = io::BufWriter::new(io::stdout());
    writeln!(handle, "{}", _current.bold().bright_green()).unwrap();
    write!(handle, "{} ", ">".bright_cyan()).unwrap();
    handle.flush().unwrap();
}
pub fn output_ver() {
    writeln!(io::BufWriter::new(io::stdout()), "MATHcmd v{}", VERSION).unwrap();
}
pub fn output_message(_message: &str) {
    let mut handle = io::BufWriter::new(io::stderr());
    if _message.starts_with("error") {
        writeln!(handle, "{}", t!(_message).bold().red()).unwrap();
    } else if _message.starts_with("warning") {
        writeln!(handle, "{}", t!(_message).bold().yellow()).unwrap();
    } else {
        writeln!(handle, "{}", t!(_message).bold().italic().bright_cyan()).unwrap();
    }
}
pub fn output_result(_result: Result<Complex, String>) {
    match _result {
        Ok(x) => println!("{} {}", "=".bold().cyan(), x.to_string().bold().cyan()),
        Err(e) => output_message(e.as_str()),
    }
}
pub fn output_point(_point: Point) {
    println!("{}", _point.to_string().bold().cyan());
}
pub fn output_function_result(_result: FunctionResult) {
    println!(
        "{} {} {}",
        _result.get_name().bold().cyan(),
        "=".bold().cyan(),
        _result.get_result().to_string().bold().cyan()
    );
}
pub fn output_help(_page: &str) {
    let help_page: String = "help.".to_string() + _page;
    println!("{}", t!(help_page.as_str()));
}
