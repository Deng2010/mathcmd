//Current page: solve
use crate::libs::expr::Expr;
use crate::{
    libs::output::{cmd_prompt, print_msg},
    print_help, print_ver,
};
use std::str::SplitWhitespace;
use std::{env, io};

pub fn solve_main() {
    env::set_var("mathcmd_page", "solve");
    let mut expr: Expr = Expr::new();
    loop {
        cmd_prompt("mathcmd->solve");
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
            _ => print_msg("error.unknown_command"),
        }
    }
}
