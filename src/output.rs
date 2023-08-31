use colored::*;
use std::io::{self, Write};
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
