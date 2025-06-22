use std::{any::type_name, fmt::Debug};

#[derive(Debug)]
pub struct Point<G1, G2> {
    x: G1,
    y: G2,
}

impl<G1, G2> Point<G1, G2> {
    pub fn new(x: G1, y: G2) -> Self {
        Self { x, y }
    }

    pub fn sauere(size: ) -> Self {
        Self { x: size, y: size }
    }

    pub fn x(&self) -> &G1 {
        &self.x
    }

    pub fn y(&self) -> &G2 {
        &self.y
    }
}

pub fn debug<G: Debug>(value: G) {
    println!(
        "the value is {:#?}. The type is: {}",
        value,
        type_name::<G>()
    );
}
