fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("Six is {:?}, None is {:#?}", six, none);

    print_only_fives(Some(1));
    print_only_fives(Some(5));
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn print_only_fives(x: Option<u8>) {
    // match x {
    //     Some(5) => println!("5"),
    //     _ => (),
    // }

    let some_u8_value = Some(0u8);
    if let Some(5) = some_u8_value {
        println!("5");
    }
}
