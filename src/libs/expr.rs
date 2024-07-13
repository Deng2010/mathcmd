use std::str::SplitWhitespace;

use crate::libs::complex::Comp;
use crate::libs::output::{print_func_res, print_msg, print_pt};
use crate::libs::point::Point;
use crate::{comp, point};

pub struct FuncRes<'a> {
    name: &'a str,
    result: Comp,
}

impl FuncRes<'_> {
    fn new(name: &str, result: Comp) -> FuncRes {
        FuncRes { name, result }
    }
    pub fn get_name(&self) -> String {
        self.name.to_string()
    }
    pub fn get_result(&self) -> Comp {
        self.result
    }
}

pub struct Expr {
    coe: Vec<Comp>,
    top: usize,
}

impl Expr {
    pub fn new() -> Expr {
        Expr {
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
            _ => print_msg("warning.equation_is_not_solvable"),
        }
        self.reset();
    }

    pub fn solve1(&mut self) {
        print_func_res(FuncRes::new("x", -self.coe[0] / self.coe[1]));
    }

    pub fn solve2(&mut self) {
        let s: Comp = -self.coe[1] / (self.coe[2] * 2.0);
        let dlt: Comp = Comp::sqrt(s * s - self.coe[0] / self.coe[2]);
        print_func_res(FuncRes::new("x1", s + dlt));
        print_func_res(FuncRes::new("x2", s - dlt));
    }

    pub fn vertex(&mut self) {
        self.update_top();
        if self.top != 2 {
            print_msg("warning.func_has_no_verts");
            return;
        }
        let x: Comp = -self.coe[1] / (self.coe[2] * 2.0);
        let y: Comp = -self.coe[0] - Comp::pow(self.coe[1] / (self.coe[2] * 2.0), 2);
        print_pt(point!(x, y));
    }

    pub fn operate(&mut self, input: &mut SplitWhitespace, op_type: char) {
        let nxt: &str = if let Some(x) = input.next() {
            x
        } else {
            print_msg("error.need_more_arguments");
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
        let (coes, num) = if let Ok(num) = arg.parse::<Comp>() {
            (0, num * pos)
        } else {
            let i: char = arg.pop().unwrap_or('0');
            if i.is_ascii_digit() {
                arg.pop();
            }
            let num: Comp = arg.parse().unwrap_or(comp!(1.0)) * pos;
            let coes: usize = i.to_string().parse::<usize>().unwrap_or(1);
            (coes, num)
        };
        self.coe[coes] += num
    }
}
