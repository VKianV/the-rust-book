use std::{any::type_name, fmt::Debug};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn debug<G>(value: &impl Debug) {
    println!(
        "the value is {:#?}. The type is: {}",
        value,
        type_name::<G>()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explorations() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
