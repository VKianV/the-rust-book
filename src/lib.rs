use std::{any::type_name, fmt::Debug};

pub fn debug<G>(value: &impl Debug) {
    println!(
        "the value is {:#?}. The type is: {}",
        value,
        type_name::<G>()
    );
}

/// Adds one to the number given.
///
/// # some Samples
///
/// ```
/// let arg = 5;
/// let answer = the_book::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 2
}
