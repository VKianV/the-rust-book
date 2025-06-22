use std::{fs::read_to_string, io};

pub fn read_username_from_file() -> Result<String, io::Error> {
    read_to_string("user_file.txt")
}
