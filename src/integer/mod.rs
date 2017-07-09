pub mod integer16;
pub use self::integer16::Integer16;

pub trait Integer<R = Self> {
    fn new() -> Self;
    fn set<T: Into<R>>(&mut self, num: T);
    fn min_value() -> Self;
    fn max_value() -> Self;
}

// Integer
