use crate::{comp, complex::Complex, output::output_result};

pub struct Cache {
    pub content: Result<Complex, String>,
    pub digit: Complex,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            content: Ok(comp!()),
            digit: comp!(),
        }
    }
    pub fn clone(&self) -> Self {
        Self {
            content: self.content.clone(),
            digit: self.digit,
        }
    }
    pub fn get_digit(&self) -> Complex {
        if self.content.is_err() {
            self.clone().output();
            return comp!();
        }
        self.digit
    }
    pub fn update(&mut self, new_content: Result<Complex, String>) {
        self.content = new_content;
    }
    pub fn update_digit(&mut self, new_digit: Complex) {
        self.digit = new_digit;
    }
    pub fn output(&mut self) {
        output_result(self.content.clone());
        if self.content.is_ok() {
            self.update_digit(self.content.clone().unwrap());
        } else {
            self.update_digit(comp!());
            self.update(Ok(comp!()));
        }
    }
}
