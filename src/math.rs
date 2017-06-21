use std::ops::{Div, Rem};

pub trait Euclidean<T>
    where T: Div + Rem
{
    /// Calculate the maximum common divisor
    fn gcd(self, p: T) -> T;
}

impl Euclidean<u8> for u8 {
    fn gcd(self, p: u8) -> u8 {
        if self % p != 0 { p.gcd(self % p) } else { p }
    }
}

impl Euclidean<u16> for u16 {
    fn gcd(self, p: u16) -> u16 {
        if self % p != 0 { p.gcd(self % p) } else { p }
    }
}

impl Euclidean<u32> for u32 {
    fn gcd(self, p: u32) -> u32 {
        if self % p != 0 { p.gcd(self % p) } else { p }
    }
}

impl Euclidean<u64> for u64 {
    fn gcd(self, p: u64) -> u64 {
        if self % p != 0 { p.gcd(self % p) } else { p }
    }
}
