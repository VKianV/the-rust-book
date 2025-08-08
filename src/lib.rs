use std::{any::type_name, fmt::Debug};

pub fn debug<G>(value: &impl Debug) {
    println!(
        "the value is {:#?}. The type is: {}",
        value,
        type_name::<G>()
    );
}
