#[macro_export]
macro_rules! own {
    ($s:expr) => { $s.to_owned().into_boxed_string() }
}
