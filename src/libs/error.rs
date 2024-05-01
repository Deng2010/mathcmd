#[macro_export]
macro_rules! err {
    ($e: expr) => {
        Err(String::from($e))
    };
}
