use std::str::SplitWhitespace;

use crate::libs::complex::Complex;
use crate::libs::output::{output_function_result, output_message, output_point};
use crate::libs::point::Point;
use crate::{comp, point};

pub struct FunctionResult<'a> {
    name: &'a str,
    result: Complex,
}

impl FunctionResult<'_> {
    fn new(name: &str, result: Complex) -> FunctionResult {
        FunctionResult { name, result }
    }
    pub fn get_name(&self) -> String {
        self.name.to_string()
    }
    pub fn get_result(&self) -> Complex {
        self.result
    }
}

pub struct Expression {
    coe: Vec<Complex>,
    top: usize,
}

impl Expression {
    pub fn new() -> Expression {
        Expression {
            coe: vec![comp!(); 10],
            top: 0,
        }
    }
    pub fn reset(&mut self) {
        self.top = 0;
        self.coe.fill(comp!());
    }
    pub fn update_top(&mut self) {
        for i in (1..5).rev() {
            if self.coe[i] != comp!() {
                self.top = i;
                return;
            }
        }
    }
    pub fn solve(&mut self) {
        self.update_top();
        for i in (1..5).rev() {
            if self.coe[i] != comp!() {
                self.top = i;
                break;
            }
        }
        match self.top {
            1 => self.solve1(),
            2 => self.solve2(),
            _ => output_message("warning.equation_is_not_solvable"),
        }
        self.reset();
    }
    pub fn solve1(&mut self) {
        let x: FunctionResult = FunctionResult::new("x", -self.coe[0] / self.coe[1]);
        output_function_result(x);
    }
    pub fn solve2(&mut self) {
        let s: Complex = -self.coe[1] / (self.coe[2] * 2.0);
        let l: Complex = self.coe[0] / self.coe[2];
        let x1: FunctionResult = FunctionResult::new("x1", s + Complex::sqrt(s * s + l));
        output_function_result(x1);
        if s * s == l {
            return;
        }
        let x2: FunctionResult = FunctionResult::new("x2", s - Complex::sqrt(s * s - l));
        output_function_result(x2)
    }
    pub fn vertex(&mut self) {
        self.update_top();
        if self.top != 2 {
            output_message("warning.func_has_no_verts");
            return;
        }
        let x: Complex = -self.coe[1] / (self.coe[2] * 2.0);
        let y: Complex = -self.coe[0] - Complex::pow(self.coe[1] / (self.coe[2] * 2.0), 2);
        output_point(point!(x, y));
    }
    pub fn operate(&mut self, input: &mut SplitWhitespace, op_type: char) {
        let nxt = input.next();
        if nxt.is_none() {
            output_message("error.need_more_arguments");
            return;
        }
        let nxt = nxt.unwrap();
        let mut arg = nxt.to_string();
        let mut _pos: isize = -1;
        let coes: usize;
        let mut num: Complex;
        if arg.parse::<Complex>().is_ok() {
            num = arg.parse().unwrap();
            coes = 0;
        } else {
            let i: char = arg.pop().unwrap_or('0');
            if i.is_alphabetic() {
                num = arg
                    .parse()
                    .unwrap_or(if arg == "-" { comp!(-1.0) } else { comp!(1.0) });
                coes = 1;
            } else {
                arg.pop();
                num = arg
                    .parse()
                    .unwrap_or(if arg == "-" { comp!(-1.0) } else { comp!(1.0) });
                coes = i.to_string().parse().unwrap();
            }
        }
        if op_type == 'r' {
            num = -num;
        }
        self.coe[coes] += num;
    }
}
