use std::{any::type_name, fmt::Debug};

pub struct Point<G1, G2> {
    pub x: G1,
    pub y: G2,
}

pub fn debug<G: Debug>(value: G) {
    println!(
        "the value is {:#?}. The type is: {}",
        value,
        type_name::<G>()
    );
}
