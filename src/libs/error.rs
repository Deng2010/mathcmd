#[macro_export]
macro_rules! err {
    ($e: expr) => {
        return Err($e.to_string())
    }
}