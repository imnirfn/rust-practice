#![allow(unused_variables)]
fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    // shadowing
    let y = 10;
    let y = y + 2;
    println!("The value of y is {}", y);

    // data types
    let num_1: u32 = "42".parse().expect("Not a number");
    let num_2: f32 = 3.1;

    println!("The value of num_2 is {}", num_2);

    let a_bool: bool = true;

}

