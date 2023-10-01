use colored::*;
use std::io::{self, Write};
use rust_i18n::t;
pub fn command_prompt(_str: String) {
    println!("{}", _str.bold().bright_green());
    print!("{} ", ">".bright_cyan());
    io::stdout().flush().unwrap();
}
pub fn output_ver(_version: &mut String) {
    println!("MATHcmd v{}", _version);
}
pub fn output_error(_type: &str) {
    println!("{}", t!(_type).bold().red());
}
pub fn output_result(_result: Result<f64, &str>) {
    match _result {
        Ok(x) => println!("{}", ("= ".to_string() + &x.to_string()).bold().cyan()),
        Err(x) => output_error(x),
    }
}
pub fn output_cache(_result: Result<f64, &str>, _digit: &mut f64) {
    output_result(_result);
    match _result {
        Ok(x) => *_digit = x,
        Err(_) => (),
    }
}
