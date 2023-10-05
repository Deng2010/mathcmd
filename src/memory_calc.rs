pub struct Memory {
    _mem: f64,
}
impl Memory {
    pub fn new() -> Memory {
        Memory { _mem: 0.0 }
    }
    pub fn add(&mut self, val: f64) {
        self._mem += val;
    }
    pub fn get(&self) -> f64 {
        self._mem
    }
    pub fn reset(&mut self) {
        self._mem = 0.0;
    }
}
