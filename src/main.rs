struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let my_stuff = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let other_stuff = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    drop(other_stuff);
    drop(my_stuff);
    println!("CustomSmartPointers created.");
}
