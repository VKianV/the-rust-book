use std::{any::type_name, fmt::Debug};

#[derive(Debug)]
pub struct Point<G> {
    x: G,
    y: G,
}

impl<G: Clone> Point<G> {
    pub fn new(x: G, y: G) -> Self {
        Self { x, y }
    }

    pub fn sauere(size: G) -> Self {
        Self {
            x: size.clone(),
            y: size,
        }
    }

    pub fn x(&self) -> &G {
        &self.x
    }

    pub fn y(&self) -> &G {
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
