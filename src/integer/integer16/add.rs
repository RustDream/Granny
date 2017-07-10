use super::Integer16;
use std::ops::Add;

impl Add for Integer16 {
    type Output = Integer16;
    fn add(self, other: Integer16) -> Integer16 {
        let _self = self.get_num() as u32;
        let _other = other.get_num() as u32;
        Integer16::Num((_self + _other) as u16)
    }
}
