#[macro_export]
macro_rules! box {
    ($s:expr) => { $s.to_owned().into_boxed_string() }
}
