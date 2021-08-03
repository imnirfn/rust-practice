fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4 {}", number);
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    }

    // using if statement in a let statement
    let condition = true;
    let num = if condition { 5 } else { 6 };

    println!("Value of num is: {}", num);

    // returning values from loops
    let mut counter = 0;
    let result = loop {
        counter += 1;

        // WTF THIS IS SOOOO COOL!!!
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Result is {}", result);

    countdown_timer();

    let a = [10,20,30,40,50];

    // looping through an array
    for i in a.iter() {
        println!("value is: {}", i);
    }
}

fn countdown_timer() {
    // reversing the range order
    for cd in (1..4).rev() {
        println!("{}!", cd);
    }
    println!("LIFTOFFF!!");
}
