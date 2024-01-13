//Current page: solve
use crate::libs::expression::Expression;
use crate::libs::output::{command_prompt, output_help, output_message, output_ver};
use std::io::stdin;
use std::str::SplitWhitespace;

pub fn solve_main() {
    let page: &str = "solve";
    let mut expr: Expression = Expression::new();
    loop {
        command_prompt("mathcmd->solve");
        let mut _input = String::new();
        stdin().read_line(&mut _input).unwrap();
        let mut input: SplitWhitespace = _input.split_whitespace();
        let __command = input.next();
        let command: &str = __command.unwrap_or("");
        match command {
            "left" | "le" | "l" => expr.operate(&mut input, 'l'),
            "right" | "ri" | "r" => expr.operate(&mut input, 'r'),
            "end" | "ed" => expr.solve(),
            "version" | "ver" | "v" => output_ver(),
            "vertex" => expr.vertex(),
            "reset" => expr.reset(),
            "exit" | "ex" => break,
            "help" | "h" => output_help(page),
            "" => continue,
            _ => output_message("error.unknown_command"),
        }
    }
}
