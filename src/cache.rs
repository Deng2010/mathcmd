use crate::output::output_result;

pub struct Cache {
    content: Result<f64, String>,
    digit: f64,
}
impl Cache {
    pub fn new(content: Result<f64, String>) -> Self {
        Self {
            content,
            digit: 0.0,
        }
    }
    pub fn get_digit(&self) -> Result<f64, String> {
        if self.content.is_err() {
            return Err("Error.The_Cache_Is_Empty".to_string())
        }
        return Ok(self.digit)
    }
    pub fn update(&mut self, new_content: Result<f64, String>) {
        self.content = new_content;
    }
    pub fn update_digit(&mut self, new_digit: f64) {
        self.digit = new_digit;
    }
    pub fn output(&mut self) {
        let _content = self.content.clone();
        output_result(_content);
        if self.content.is_ok() {
            let _content = self.content.clone();
            let _digit = _content.unwrap();
            self.update_digit(_digit);
        } else {
            self.update_digit(0.0);
            self.update(Err("Error.The_Cache_Is_Empty".to_string()));
        }
    }
}