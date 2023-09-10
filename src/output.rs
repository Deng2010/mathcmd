use colored::*;
use std::io::{self, Write};
use rust_i18n::t;
pub fn command_prompt(_str: String) {
    println!("{}", _str.bold().bright_green());
    print!("{} ", ">".bright_cyan());
    io::stdout().flush().unwrap();
}
pub fn output_cache(_cache: f64) {
    println!("{}", ("= ".to_string() + &_cache.to_string()).bold().cyan());
}
pub fn output_ver(_version: &mut String) {
    println!("MATHcmd v{}", _version);
}
pub fn output_error(_type: &str) {
    println!("{}", t!(_type).bold().red());
}
pub fn output_exit(_code: i32) {
    println!("{}", t!("Exit", code = _code).bold().blue());
}
