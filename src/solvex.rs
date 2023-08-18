pub mod solvex {
    use colored::*;
    use std::io::{self, Write};
    pub fn solvex_mode() {
        let mut coe: [f64; 2] = [0.0, 0.0];
        loop {
            println!(
                "{}{}{}",
                "mathcmd".bright_green(),
                "->".bright_cyan(),
                "solvex".bright_green()
            );
            print!("{} ", ">".bright_cyan());
            io::stdout().flush().unwrap();
            let mut _input = String::new();
            io::stdin()
                .read_line(&mut _input)
                .expect("ERROR: Unknown command!");
            let mut input = _input.split_whitespace();
            let __command = input.next();
            if __command.is_none() {
                continue;
            }
            let command = __command.unwrap();
            match command {
                "left" | "le" | "l" => {
                    let mut item: String = input.next().unwrap().to_string();
                    let i = item.pop().unwrap();
                    if i == 'x' {
                        let num: f64;
                        if item.is_empty() {
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
                }
                "right" | "ri" | "r" => {
                    let mut item: String = input.next().unwrap().to_string();
                    let i = item.pop().unwrap();
                    if i == 'x' {
                        let num: f64;
                        if item.is_empty() {
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
                }
                "end" | "ed" => {
                    println!("{}", coe[0] / coe[1]);
                    coe[0] = 0.0;
                    coe[1] = 0.0;
                }
                "exit" | "ex" => return,
                _default => println!("ERROR: Unknown command!"),
            }
        }
    }
}
