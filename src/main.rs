use std::{fs::File, io::ErrorKind};

fn main() {
    let greeting_file_result = File::open("the_devil_file.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("the_devil_file.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("problem creating the file {e:?}"),
            },
            _ => {
                panic!("error opening the file: {error:?}");
            }
        },
    };
}
