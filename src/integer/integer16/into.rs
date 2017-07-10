use super::Integer16;

/// # Example
/// ```Rust
/// impl Into<Integer16> for u32 {
///     fn into(self) -> Integer16 {
///         Integer16::Num(self as u16)
///     }
/// }
/// ```
macro_rules! into_integer16_for_ux {
    ($($t:ty)*) => ($(
        impl Into<Integer16> for $t {
            fn into(self) -> Integer16 {
                Integer16::Num(self as u16)
            }
        }
    )*)
}

into_integer16_for_ux!{u8 u16 u32 u64 usize}

/// # Example
/// ```Rust
/// impl Into<Integer16> for i32 {
///     fn into(self) -> Integer16 {
///         let mut _self = 0;
///         if self < 0 {
///             _self = (u16::max_value() as i32 + (self as i16) as i32 + 1) as u16
///         } else {
///             _self = self as u16
///         }
///         Integer16::Num(_self)
///     }
/// }
/// ```
macro_rules! into_integer16_for_ix {
    ($($t:ty)*) => ($(
        impl Into<Integer16> for $t {
            fn into(self) -> Integer16 {
                let mut _self = 0;
                if self < 0 {
                    _self = (u16::max_value() as i32 + (self as i16) as i32 + 1) as u16
                } else {
                    _self = self as u16
                }
            Integer16::Num(_self)
            }
        }
    )*)
}

into_integer16_for_ix!{i8 i16 i32 i64 isize}
