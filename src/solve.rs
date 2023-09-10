use std::io;
use std::str::SplitWhitespace;

use crate::output::{command_prompt, output_error};

fn _operate(coe: &mut Vec<f64>, input: &mut SplitWhitespace, op_type: String) {
    let mut item: String = input.next().unwrap().to_string();
    let i: char = item.pop().unwrap();
    let coes: usize;
    let mut num: f64;
    if i == 'x' {
        num = 1.0;
        if !item.is_empty() {
            num = item.parse().unwrap();
        }
        coes = 1;
    } else {
        item.push(i);
        num = item.parse().unwrap();
        coes = 0;
    }
    if (coes == 0) ^ (op_type == "r") {
        num = -num;
    }
    coe[coes] += num;
}

pub fn solve_function_one_one() {
    // 1. 输入系数
    let mut coe: Vec<f64> = vec![0.0, 0.0];  
    // 2. 输入操作符
    let mut _input = String::new();
    loop {
        command_prompt("mathcmd->solve".to_string());
        io::stdin()
            .read_line(&mut _input)
            .unwrap();
        let mut input: SplitWhitespace = _input.split_whitespace();
        let __command = input.next();
        if __command.is_none() {
            continue;
        }
        let command: &str = __command.unwrap();
        match command {
            "left" | "le" | "l" | "right" | "ri" | "r" => {
                _operate(&mut coe, &mut input, command[0..1].to_string());
            }
            "end" | "ed" => {
                println!("{}", coe[0] / coe[1]);
                coe[0] = 0.0;
                coe[1] = 0.0;
            }
            "exit" | "ex" => return,
            _default => output_error("Error.Unknown_command"),
        }
        _input = String::new();
    }
}
