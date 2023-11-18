use crate::comp;
use crate::complex::Complex;
use crate::output::{command_prompt, output_function_result, output_help, output_message};
use std::io::stdin;
use std::str::SplitWhitespace;

pub struct FunctionResult {
    name: String,
    result: Complex,
}

impl FunctionResult {
    fn new(name: String, result: Complex) -> FunctionResult {
        FunctionResult { name, result }
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_result(&self) -> Complex {
        self.result.clone()
    }
}
struct Expression {
    coe: Vec<Complex>,
    top: usize,
}

impl Expression {
    fn new() -> Expression {
        Expression {
            coe: vec![comp!(); 10],
            top: 0,
        }
    }
    fn reset(&mut self) {
        self.top = 0;
        self.coe.fill(comp!());
    }
    fn solve(&mut self) {
        match self.top {
            1 => self.solve1(),
            2 => self.solve2(),
            _ => output_message("Warning.Equation_Is_Not_Solvable"),
        }
        self.reset();
    }
    fn solve1(&mut self) {
        let x: FunctionResult = FunctionResult::new("x".to_owned(), -self.coe[0] / self.coe[1]);
        output_function_result(x);
    }
    fn solve2(&mut self) {
        let s: Complex = -self.coe[1] / (self.coe[2] * 2.0);
        let l: Complex = self.coe[0] / self.coe[2];
        let x1: FunctionResult = FunctionResult::new("x1".to_owned(), s + Complex::sqrt(s * s - l));
        output_function_result(x1);
        if s * s != l {
            let x2: FunctionResult =
                FunctionResult::new("x2".to_owned(), s - Complex::sqrt(s * s - l));
            output_function_result(x2)
        };
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
    let nxt = nxt.unwrap();
    let mut arg = nxt.to_string();
    let mut _pos: isize = -1;
    let coes: usize;
    let mut num: f64;
    if arg.parse::<f64>().is_ok() {
        num = arg.parse().unwrap();
        coes = 0;
    } else {
        let i: char = arg.pop().unwrap_or('0');
        if i == 'x' {
            num = arg.parse().unwrap_or(1.0);
            coes = 1;
        } else {
            arg.pop();
            num = arg.parse().unwrap_or(1.0);
            coes = i.to_string().parse().unwrap();
        }
    }
    if op_type == "r" {
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
