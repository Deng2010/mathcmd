use std::io::{self, Write};
fn main() {
    let __version = String::from("0.0.3");
    println!("MATHcmd v{}", __version);
    loop {
        print!("mathcmd> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("ERROR: Unknown command!");
        let mut input_split = input.split_whitespace();
        let command = input_split.next().unwrap();
        if command == "+" {
            let a: f64 = input_split.next().unwrap().parse().unwrap();
            let b: f64 = input_split.next().unwrap().parse().unwrap();
            println!("{}", a + b);
        } else if command == "-" {
            let a: f64 = input_split.next().unwrap().parse().unwrap();
            let b: f64 = input_split.next().unwrap().parse().unwrap();
            println!("{}", a - b);
        } else if command == "*" {
            let a: f64 = input_split.next().unwrap().parse().unwrap();
            let b: f64 = input_split.next().unwrap().parse().unwrap();
            println!("{}", a * b);
        } else if command == "/" {
            let a: f64 = input_split.next().unwrap().parse().unwrap();
            let b: f64 = input_split.next().unwrap().parse().unwrap();
            println!("{}", a / b);
        } else if command == "solvex" {
            solvex_cmd();
        } else if command == "exit" {
            return;
        } else {
            println!("Error: {} is undefined!", command);
        }
    }
}
pub fn solvex_cmd() {
    let mut coe: [f64; 2] = [0.0, 0.0];
    loop {
        print!("mathcmd->solvex> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("ERROR: Unknown command!");
        let mut input_split = input.split_whitespace();
        let command = input_split.next().unwrap();
        if command == "left" {
            let mut item: String = input_split.next().unwrap().to_string();
            let i = item.pop().unwrap();
            if i == 'x' {
                let num: f64;
                if item.len() == 0 {
                    num = 1.0;
                } else {
                    num = item.parse().unwrap();
                }
                coe[1] += num;
            } else {
                item.push(i);
                let num: f64 = item.parse().unwrap();
                coe[0] -= num;
            }
        } else if command == "right" {
            let mut item: String = input_split.next().unwrap().to_string();
            let i = item.pop().unwrap();
            if i == 'x' {
                let num: f64;
                if item.len() == 0 {
                    num = 1.0;
                } else {
                    num = item.parse().unwrap();
                }
                coe[1] -= num;
            } else {
                item.push(i);
                let num: f64 = item.parse().unwrap();
                coe[0] += num;
            }
        } else if command == "end" {
            println!("{}", coe[0] / coe[1]);
            coe[0] = 0.0;
            coe[1] = 0.0;
        } else if command == "exit" {
            return;
        }
    }
}
