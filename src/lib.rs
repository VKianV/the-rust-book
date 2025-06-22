use std::{any::type_name, fmt::Debug};

pub fn find_largest(input: &[i32]) -> &i32 {
    let mut largest = &input[0];
    for number in input {
        if number > largest {
            largest = number;
        }
    }
    largest
}

pub fn debug<T: Debug>(value: T) {
    println!(
        "the value is {:#?}. The type is: {}",
        value,
        type_name::<T>()
    );
}
