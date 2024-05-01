use std::str::SplitWhitespace;

use crate::libs::complex::Complex;
use crate::libs::output::{print_function_result, print_message, print_point};
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
        match self.top {
            1 => self.solve1(),
            2 => self.solve2(),
            _ => print_message("warning.equation_is_not_solvable"),
        }
        self.reset();
    }
    pub fn solve1(&mut self) {
        let x: FunctionResult = FunctionResult::new("x", -self.coe[0] / self.coe[1]);
        print_function_result(x);
    }
    pub fn solve2(&mut self) {
        let s: Complex = -self.coe[1] / (self.coe[2] * 2.0);
        let dlt: Complex = s * s - self.coe[0] / self.coe[2];
        let (x1, x2): (FunctionResult, FunctionResult) = (
            FunctionResult::new(if dlt == 0.0 { "x" } else { "x1" }, s + Complex::sqrt(dlt)),
            FunctionResult::new("x2", s - Complex::sqrt(dlt)),
        );
        print_function_result(x1);
        if dlt != 0.0 {
            print_function_result(x2);
        }
    }
    pub fn vertex(&mut self) {
        self.update_top();
        if self.top != 2 {
            print_message("warning.func_has_no_verts");
            return;
        }
        let x: Complex = -self.coe[1] / (self.coe[2] * 2.0);
        let y: Complex = -self.coe[0] - Complex::pow(self.coe[1] / (self.coe[2] * 2.0), 2);
        print_point(point!(x, y));
    }
    pub fn operate(&mut self, input: &mut SplitWhitespace, op_type: char) {
        let nxt: &str = if let Some(x) = input.next() {
            x
        } else {
            print_message("error.need_more_arguments");
            return;
        };
        let mut arg: String = nxt.to_string();
        let pos: bool = if let Some(x) = arg.strip_prefix('-') {
            arg = x.to_string();
            op_type == 'r'
        } else {
            op_type == 'l'
        };
        let pos: f64 = if pos { 1.0 } else { -1.0 };
        let (coes, num) = if let Ok(num) = arg.parse::<Complex>() {
            (0, num * pos)
        } else {
            let i: char = arg.pop().unwrap_or('0');
            if i.is_ascii_digit() {
                arg.pop();
            }
            let num: Complex = arg.parse().unwrap_or(comp!(1.0)) * pos;
            let coes: usize = i.to_string().parse::<usize>().unwrap_or(1);
            (coes, num)
        };
        self.coe[coes] += num
    }
}
