fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    // shadowing
    let y = 10;
    let y = y + 2;
    // the value will be added
    println!("The value of y is {}", y);

    // data types
    let a = 2.0; // f64 (default float)
    let num_1: u32 = "42".parse().expect("Not a number");
    let num_2: f32 = 3.1;

    println!("The value of num_2 is {}", num_2);

    let a_bool: bool = true;
    let heart_eyed_cat = '😻';
    println!("Rust support emoji! {}", heart_eyed_cat);

    // tuple (rarely will be use)
    let tup = (12, 24.2,1);
    
    // destructuring
    let (d , e, f) = tup;
    println!("d: {}. e: {}. f: {}", d, e, f);


    // array (use most of the time)
    // explicit declaration
    let months: [&str; 12] = ["January", "February", "March", "April", "May", "June", 
    "July", "August", "September", "October", "November", "December"];

    // non-explicity declare var array
    // let months = ["January", "February", "March", "April", "May", "June", "July",
    //         "August", "September", "October", "November", "December"];

    // accessing content of the array using indexing
    println!("Month, {}", months[1]);
}