#[macro_export]
macro_rules! terminal_input {
    () => {
        let mut _input: String = String::new();
        stdin().read_line(&mut _input).unwrap();
        _input.split_whitespace();
    };
}
