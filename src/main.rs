use crate::back_of_house::the_module_file::TheVariable;
use std::any::type_name;
use std::fmt::Debug;

pub mod back_of_house;

fn main() {
    let var = TheVariable { x: 10, y: 20 };
    debug(var);
}

#[warn(dead_code)]
fn debug<T: Debug>(value: T) {
    println!(
        "the value is {:#?}. The type is: {}",
        value,
        type_name::<T>()
    );
}
