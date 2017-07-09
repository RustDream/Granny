extern crate granny;
use granny::{Integer, Integer16};
fn main() {
    let mut foo = Integer16::new();
    let mut bar = Integer16::new();
    foo.set(-1);
    bar.set(-1);
    println!("{:?}", foo + bar);
}
