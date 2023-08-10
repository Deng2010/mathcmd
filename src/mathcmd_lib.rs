pub mod core {
    pub fn is_num(s1: String) -> bool {
        let s = &s1[0..1];
        match s == "0"
            || s == "1"
            || s == "2"
            || s == "3"
            || s == "4"
            || s == "5"
            || s == "6"
            || s == "7"
            || s == "8"
            || s == "9"
            || s == "-"
        {
            true => return true,
            false => (),
        }
        return false;
    }
}
