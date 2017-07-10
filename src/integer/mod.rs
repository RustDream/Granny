pub mod integer16;
pub use self::integer16::Integer16;

pub trait Integer<R = Self> {
    fn new() -> Self;
    fn set<T: Into<R>>(&mut self, num: T);
    fn min_value() -> Self;
    fn max_value() -> Self;
}

/// # Example
/// ```Rust
/// impl Integer for Integer16 {
///     fn new() -> Self {
///         Integer16::Null
///     }
/// 
///     fn set<T: Into<Integer16>>(&mut self, num: T) {
///         *self = num.into();
///     }
/// 
///     fn min_value() -> Self {
///         Integer16::Num(0)
///     }
///
///     fn max_value() -> Self {
///         Integer16::Num(65535)
///     }
/// }
/// ```
macro_rules! integer_impl {
    ($($t:ident)*) => ($(
        impl Integer for $t {
            fn new() -> Self {
                $t::Null
            }

            fn set<T: Into<$t>>(&mut self, num: T) {
                *self = num.into();
            }

            fn min_value() -> Self {
                $t::Num(0)
            }
            
            fn max_value() -> Self {
                let foo = -1;
                foo.into()
            }
        }
    )*)
}

integer_impl!{Integer16}

