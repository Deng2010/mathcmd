use crate::modules::{calc::calc_main as calc, geo::geo_main as geo, solve::solve_main as solve};
use crate::{
    libs::output::{cmd_prompt, print_msg},
    print_help, print_ver,
};

use std::env;
use std::io::stdin;
pub fn mathcmd_main() {
    loop {
        cmd_prompt("mathcmd");
        let mut input: String = String::new();
        stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "calc" => calc(),
            "solve" => solve(),
            "geo" => geo(),
            "exit" | "ex" => break,
            "version" | "ver" | "v" => print_ver!(),
            "" => (),
            "help" | "h" => print_help!(),
            _ => print_msg("error.unknown_command"),
        }
    }
}
