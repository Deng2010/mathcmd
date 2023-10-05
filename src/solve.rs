use std::io::stdin;
use std::str::SplitWhitespace;

use crate::output::{
    command_prompt, 
    output_error
};

struct Expression {
    coe: Vec<f64>,
}

impl Expression {
    fn new() -> Expression {
        Expression { 
            coe: vec![0.0, 0.0],
        }
    }

    fn solve(&mut self) {
        println!("{}", self.coe[0] / self.coe[1]);
        self.coe = vec![0.0, 0.0];
    }
}

fn _operate(expr: &mut Expression, input: &mut SplitWhitespace, op_type: &str) {
    let nxt = input.next();
    if nxt.is_none() {
        output_error("Error.Need_More_Arguments");
        return;
    }
    let mut nxt = nxt.unwrap().to_string();
    let i: char = nxt.pop().unwrap();
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
    expr.coe[coes] += num;
}

pub fn solve_function_one_one() {
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
            "left" | "le" | "l" | "right" | "ri" | "r" => 
                _operate(&mut expr, &mut input, &command[0..1]),
            "end" | "ed" => 
                expr.solve(),
            "exit" | "ex" => 
                break,
            "" => 
                continue,
            _ => 
                output_error("Error.Unknown_Command"),
        }
    }
}
