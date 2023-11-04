use std::io::stdin;
use std::str::SplitWhitespace;

use crate::output::{command_prompt, output_help, output_message};

struct Expression {
    coe: Vec<f64>,
    top: usize,
}

impl Expression {
    fn new() -> Expression {
        Expression {
            coe: vec![0.0; 10],
            top: 0,
        }
    }
    fn reset(&mut self) {
        self.top = 0;
        self.coe.fill(0.0);
    }
    fn solve(&mut self) {
        match self.top {
            1 => self.solve1(),
            _ => output_message("Warning.Equation_Is_Not_Solvable"),
        }
        self.reset();
    }
    fn solve1(&mut self) {
        println!("{}", self.coe[0] / self.coe[1]);
    }
    fn modify(&mut self, coes: usize, num: f64) {
        self.top = if self.top < coes { coes } else { self.top };
        self.coe[coes] += num;
    }
}

fn _operate(expr: &mut Expression, input: &mut SplitWhitespace, op_type: &str) {
    let nxt = input.next();
    if nxt.is_none() {
        output_message("Error.Need_More_Arguments");
        return;
    }
    let mut nxt = nxt.unwrap().to_string();
    let i: char = nxt.pop().unwrap_or('0');
    let coes: usize;
    let mut num: f64;
    if i == 'x' {
        num = nxt.parse().unwrap_or(1.0);
        coes = 1;
    } else {
        nxt.push(i);
        num = nxt.parse().unwrap();
        coes = 0;
    }
    if (coes == 0) ^ (op_type == "r") {
        num = -num;
    }
    expr.modify(coes, num);
}

pub fn solve_function() {
    // 1. 输入系数
    let mut expr: Expression = Expression::new();
    loop {
        command_prompt("mathcmd->solve");
        let mut _input = String::new();
        stdin().read_line(&mut _input).unwrap();
        let mut input: SplitWhitespace = _input.split_whitespace();
        let __command = input.next();
        let command: &str = __command.unwrap_or("");
        match command {
            "left" | "le" | "l" | "right" | "ri" | "r" => {
                _operate(&mut expr, &mut input, &command[0..1])
            }
            "end" | "ed" => expr.solve(),
            "exit" | "ex" => break,
            "help" | "h" => output_help("solve"),
            "" => continue,
            _ => output_message("Error.Unknown_Command"),
        }
    }
}
