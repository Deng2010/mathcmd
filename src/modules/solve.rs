//Current page: solve
use crate::libs::expression::Expression;
use crate::{
    libs::output::{command_prompt, print_message},
    print_help, print_ver,
};
use std::{env, io};
use std::str::SplitWhitespace;

pub fn solve_main() {
    env::set_var("mathcmd_page", "solve");
    let mut expr: Expression = Expression::new();
    loop {
        command_prompt("mathcmd->solve");
        let mut _input = String::new();
        io::stdin().read_line(&mut _input).unwrap();
        let mut input: SplitWhitespace = _input.split_whitespace();
        let __command = input.next();
        let command: &str = __command.unwrap_or("");
        match command {
            "left" | "le" | "l" => expr.operate(&mut input, 'l'),
            "right" | "ri" | "r" => expr.operate(&mut input, 'r'),
            "end" | "ed" => expr.solve(),
            "vertex" => expr.vertex(),
            "reset" => expr.reset(),
            "exit" | "ex" => break,
            "version" | "ver" | "v" => print_ver!(),
            "help" | "h" => print_help!(),
            "" => continue,
            _ => print_message("error.unknown_command"),
        }
    }
}
