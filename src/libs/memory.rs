use crate::libs::complex::Complex;

pub struct Memory {
    mem: Complex,
}
impl Memory {
    pub fn new() -> Memory {
        Memory {
            mem: Complex::new(0.0, 0.0),
        }
    }
    pub fn get(&self) -> Complex {
        self.mem
    }
    pub fn reset(&mut self) -> Result<Complex, String> {
        self.mem = Complex::new(0.0, 0.0);
        Err(String::from("none"))
    }
    pub fn get_reset(&mut self) -> Result<Complex, String> {
        let tmp: Complex = self.mem;
        self.mem = Complex::new(0.0, 0.0);
        Ok(tmp)
    }
    pub fn add(&mut self, rhs: Complex) -> Result<Complex, String> {
        self.mem += rhs;
        Err(String::from("none"))
    }
    pub fn sub(&mut self, rhs: Complex) -> Result<Complex, String> {
        self.mem += rhs;
        Err(String::from("none"))
    }
}
