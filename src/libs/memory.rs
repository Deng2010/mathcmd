use std::ops::{AddAssign, SubAssign};

use crate::{comp, libs::complex::Complex};

pub struct Memory {
    mem: Complex,
}
impl Memory {
    pub fn new() -> Memory {
        Memory { mem: comp!() }
    }
    pub fn get(&self) -> Complex {
        self.mem
    }
    pub fn reset(&mut self) {
        self.mem = comp!();
    }
}
impl AddAssign<Complex> for Memory {
    fn add_assign(&mut self, rhs: Complex) {
        self.mem += rhs;
    }
}
impl SubAssign<Complex> for Memory {
    fn sub_assign(&mut self, rhs: Complex) {
        self.mem -= rhs;
    }
}
