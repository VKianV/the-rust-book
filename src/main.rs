fn main() {
    // let mut num = 5;

    let mut num2 = 50;

    // let r1 = &raw const num;
    // let r2 = &raw mut num;

    let r3 = &num2;
    let r4 = &mut num2;

    // unsafe {
    //     println!("r1 is: {:?}", *
    //
    //     r1);
    //     println!("r2 is: {:?}", *r2);
    // }

    println!("r3 is: {}", r3);
    println!("r4 is: {}", r4);
}
