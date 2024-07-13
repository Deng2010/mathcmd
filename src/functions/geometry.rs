use crate::libs::complex::Comp;

pub fn heron_formula(a: Comp, b: Comp, c: Comp) -> Comp {
    let s: Comp = (a + b + c) / 2.0;
    Comp::sqrt(s * (s - a) * (s - b) * (s - c))
}
