use crate::{complex::Complex, modules::solve::FunctionResult, version};
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
    if _message.starts_with("Error") {
        println!("{}", t!(_message).bold().red());
    } else if _message.starts_with("Warning") {
        println!("{}", t!(_message).bold().yellow());
    } else {
        println!("{}", t!(_message).bold().italic().bright_cyan());
    }
}
pub fn output_result(_result: Result<Complex, String>) {
    match _result {
        Ok(x) => println!("{}", ("= ".to_string() + &x.to_string()).bold().cyan()),
        Err(err) => output_message(&err),
    }
}
pub fn output_function_result(_result: FunctionResult) {
    println!(
        "{}",
        (_result.get_name() + " = " + &_result.get_result().to_string())
            .bold()
            .cyan()
    );
}
pub fn output_help(_page: &str) {
    let help_page = "Help.".to_string() + _page;
    println!("{}", t!(&help_page));
}
