mod into;
mod add;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Integer16 {
    Num(u16),
    Null,
}

impl Integer16 {
    fn get_num(&self) -> u16 {
        match *self {
            Integer16::Num(e) => e,
            _ => 0,
        }
    }
}




