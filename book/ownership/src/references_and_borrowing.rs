fn main() {
    // variables/references are immutable by default
    // means we need to delcare it mutable
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);

    // dangling pointer reference
    let reference_to_nothing = dangle();
    println!("{}", reference_to_nothing);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn dangle() -> String {
    let s = String::from("holla");

    s
}
