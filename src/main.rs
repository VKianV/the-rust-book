#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let number = SpreadsheetCell::Int(3);
    let text = SpreadsheetCell::Text(String::from("blue"));
    let float = SpreadsheetCell::Float(10.12);

    let row = vec![number, text, float];

    for i in &row {
        println!("{:?}", i);
    }
}
