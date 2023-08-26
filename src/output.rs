use colored::*;
use std::io::{self, Write};
map
pub fn command_prompt() {
    println!("{}", "mathcmd".bright_green());
    print!("{} ", ">".bright_cyan());
    io::stdout().flush().unwrap();
}
pub fn output_cache(_cache: f64) {
    println!("{} {}", "=".bright_cyan(), _cache);
}
pub fn output_ver(_version: &mut String) {
    println!("MATHcmd v{}", _version);
}
