use colored::*;
use std::io::{self, Write};
use crate::data::version;
pub fn command_prompt(_current: &str) {
    println!("{}", _current.bold().bright_green());
    print!("{} ", ">".bright_cyan());
    io::stdout().flush().unwrap();
}
pub fn output_ver() {
    println!("MATHcmd v{}", version());
}
pub fn output_error(_type: &str) {
    println!("{}", t!(_type).bold().red());
}
pub fn output_result(_result: Result<f64, &str>) {
    match _result {
        Ok(x) => println!("{}", ("= ".to_string() + &x.to_string()).bold().cyan()),
        Err(err) => output_error(err),
    }
}
pub fn output_help(_page: &str) {
    let help_page = "Help.".to_string() + _page;
    println!("{}", t!(&help_page));
}
pub fn output_cache(_cache: Result<f64, &str>, _digit: &mut f64) {
    output_result(_cache);
    match _cache {
        Ok(x) => *_digit = x,
        _ => (),
    }
}
