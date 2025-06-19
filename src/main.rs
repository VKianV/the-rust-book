use std::{fs::File, io::ErrorKind};

fn main() {
    let greeting_file =
        File::open("hello.txt").expect("the file must be included in the application");
}
