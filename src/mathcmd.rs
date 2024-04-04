use crate::libs::output::{command_prompt, output_help, output_message, output_ver};
use crate::modules::{calc::calc_main as calc, geo::geo_main as geo, solve::solve_main as solve};

use std::io::stdin;
pub fn mathcmd_main() {
    let page: &str = "main";
    loop {
        command_prompt("mathcmd");
        let mut input: String = String::new();
        stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "calc" => calc(),
            "solve" => solve(),
            "geo" => geo(),
            "exit" | "ex" => break,
            "version" | "ver" | "v" => output_ver(),
            "" => (),
            "help" | "h" => output_help(page),
            _ => output_message("error.unknown_command"),
        }
    }
}
