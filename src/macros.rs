#[macro_export]
#[allow(clippy::crate_in_macro_def)]
///快捷实现run接口，适用于已实现了BookParse trait 和 Download trait的方法。
macro_rules! run_impl {
    ($i:ident) => {impl crate::traits::Run for $i{}};
}