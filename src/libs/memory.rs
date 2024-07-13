use crate::libs::complex::Comp;

pub struct Memory {
    mem: Comp,
}
impl Memory {
    pub fn new() -> Memory {
        Memory {
            mem: Comp::new(0.0, 0.0),
        }
    }
    pub fn get(&self) -> Comp {
        self.mem
    }
    pub fn reset(&mut self) -> Result<Comp, String> {
        self.mem = Comp::new(0.0, 0.0);
        Err(String::from("none"))
    }
    pub fn get_reset(&mut self) -> Result<Comp, String> {
        let tmp: Comp = self.mem;
        self.mem = Comp::new(0.0, 0.0);
        Ok(tmp)
    }
    pub fn add(&mut self, rhs: Comp) -> Result<Comp, String> {
        self.mem += rhs;
        Err(String::from("none"))
    }
    pub fn sub(&mut self, rhs: Comp) -> Result<Comp, String> {
        self.mem += rhs;
        Err(String::from("none"))
    }
}
