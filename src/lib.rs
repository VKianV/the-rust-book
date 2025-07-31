use std::{any::type_name, fmt::Debug};

pub fn debug<G>(value: &impl Debug) {
    println!(
        "the value is {:#?}. The type is: {}",
        value,
        type_name::<G>()
    );
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explorations() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }
}
