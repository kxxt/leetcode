pub use paste::paste;

#[macro_export]
macro_rules! test_eq {
    ($id: literal, $func:expr, $expected: expr) => {
        paste! {#[test] fn [<test_case_ $id>]() {
            assert_eq!($func, $expected);
        }}
    };
}

#[macro_export]
macro_rules! tests {
    ($($t:tt)*) => {
        #[cfg(test)]
        mod tests {
            use super::*;
            $($t)*
        }
    };
}
