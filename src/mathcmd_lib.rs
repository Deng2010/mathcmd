pub mod core {
    pub fn is_num(s1: &str) -> bool {
        if s1.len() == 1 {
            match s1 {
                "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "-" => return true,
                _default => return false,
            }
        }
        for i in 0..s1.len() {
            let s = &s1[i..i];
            match s {
                "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "-" => (),
                _default => return false,
            }
        }
        return true;
    }
}
