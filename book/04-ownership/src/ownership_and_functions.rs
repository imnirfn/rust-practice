fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s); // prints out error
                          // bcs s is no longer valid
                          // since its moved into the fn

    let x = 5;
    makes_copy(x);
    println!("{}", x); // would be no problem bcs
                       // i32 has a Copy trait

    let s1 = gives_ownership(); // transfering ownership
    println!("{}", s1);         // using returning values

    let s2 = String::from("huii");
    let s3 = takes_and_gives_back(s2);
    println!("{}", s3);

    let c1 = String::from("heyyo!");
    let len = calculate_length(&c1); // exmple of reference
                                     // refer value without
                                     // taking ownership
    println!("The length of '{}' is {}.", c1, len);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let s = String::from("holla");
    s
}

fn takes_and_gives_back(some_string: String) -> String {
    some_string
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
