use std::{any::type_name, fmt::Debug};

pub fn find_largest<T: PartialOrd>(input: &[T]) -> &T {
    let mut largest = &input[0];
    for number in input {
        if number > largest {
            largest = number;
        }
    }
    largest
}

pub fn debug<G: Debug>(value: G) {
    println!(
        "the value is {:#?}. The type is: {}",
        value,
        type_name::<G>()
    );
}
