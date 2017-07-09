use super::Integer;
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Integer16 {
    Num(u16),
    Null,
}

impl Into<Integer16> for u16 {
    fn into(self) -> Integer16 {
        Integer16::Num(self)
    }
}

impl Into<Integer16> for u32 {
    fn into(self) -> Integer16 {
        Integer16::Num(self as u16)
    }
}

impl Into<Integer16> for u64 {
    fn into(self) -> Integer16 {
        Integer16::Num(self as u16)
    }
}

impl Into<Integer16> for i16 {
    fn into(self) -> Integer16 {
        let mut _self = 0;
        if self < 0 {
            _self = (u16::max_value() as i32 + self as i32 + 1) as u16
        } else {
            _self = self as u16
        }
        Integer16::Num(_self)
    }
}

impl Into<Integer16> for i32 {
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

impl Into<Integer16> for i64 {
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

impl Integer16 {
    fn get_num(&self) -> u16 {
        match *self {
            Integer16::Num(e) => e,
            _ => 0,
        }
    }
}

impl Integer for Integer16 {
    fn new() -> Self {
        Integer16::Null
    }

    fn set<T: Into<Integer16>>(&mut self, num: T){
        *self = num.into();
    }

    fn min_value() -> Self{
        Integer16::Num(0)
    }
    fn max_value() -> Self{
        Integer16::Num(65535)
    }
}

impl Add for Integer16 {
    type Output = Integer16;
    fn add(self, other: Integer16) -> Integer16 {
        let _self = self.get_num() as u32;
        let _other = other.get_num() as u32;
        Integer16::Num((_self + _other) as u16)
    }
}

