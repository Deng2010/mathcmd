pub mod core {
    pub fn is_num(s1: &str) -> bool {
        let s2 = s1.to_string();
        let s = &s2[0..1];
        match s == "0"
            || s == "1"
            || s == "2"
            || s == "3"
            || s == "4"
            || s == "5"
            || s == "6"
            || s == "7"
            || s == "8"
            || s == "9" {
            true => return true,
            false => (),
        }
        return false;
    }
}
