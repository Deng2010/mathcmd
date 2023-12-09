use crate::{complex::Complex, modules::solve::FunctionResult, point::Point, version};
use colored::*;
use std::io::{self, Write};
pub fn command_prompt(_current: &str) {
    println!("{}", _current.bold().bright_green());
    print!("{} ", ">".bright_cyan());
    io::stdout().flush().unwrap();
}
pub fn output_ver() {
    println!("MATHcmd v{}", version!());
}
pub fn output_message(_message: &str) {
    let _message = _message.to_ascii_lowercase();
    if _message.starts_with("error") {
        println!("{}", t!(_message.as_str()).bold().red());
    } else if _message.starts_with("warning") {
        println!("{}", t!(_message.as_str()).bold().yellow());
    } else {
        println!("{}", t!(_message.as_str()).bold().italic().bright_cyan());
    }
}
pub fn output_result(_result: Result<Complex, String>) {
    match _result {
        Ok(x) => println!("{} {}", "=".bold().cyan(), x.to_string().bold().cyan()),
        Err(err) => output_message(err.as_str()),
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
