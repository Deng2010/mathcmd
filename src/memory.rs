use crate::{comp, complex::Complex};

pub struct Memory {
    _mem: Complex,
}
impl Memory {
    pub fn new() -> Memory {
        Memory {
            _mem: comp!(0.0, 0.0),
        }
    }
    pub fn add(&mut self, val: Complex) {
        self._mem += val;
    }
    pub fn get(&self) -> Complex {
        self._mem
    }
    pub fn reset(&mut self) {
        self._mem = comp!(0.0, 0.0);
    }
}
