use std::{
    fs::File,
    io::{self, Read},
};

pub fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("user_file.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
