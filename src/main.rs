use std::ops::Add;

#[derive(Debug)]
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

fn main() {
    let (ten_milis, one_met) = (Millimeters(10), Meters(1));
    let result = ten_milis + one_met;

    println!("Your value should be such a thing: {:?}", &result);
}
