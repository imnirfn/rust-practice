fn main() {
    // string literal (means it is hardcoded string)
    // this is not good bcs its immutable since hardcoded
    // hence cannot be mutated
    let _s = "hello"; // _ is idc if this not used

    // this can be mutated
    let mut w = String::from("hello");
    w.push_str(", people!");

    println!("{}", w);

    // Integers are simple values and stored on the stack
    // so in this case y is a copy of 5
    // it is valid bcs its only stored the value on the stack
    // a simple values or primitive values have a copy traits
    let x = 5;
    let y = x;

    let tup: (u32, bool) = (210, false);
    let tup_clone = tup;
    let (tup_x, tup_y) = tup_clone;

    println!("x = {}, y = {}", x, y);
    println!("tup_x = {}, tup_y = {}", tup_x, tup_y);

    // example of move
    // can do stuff with s bcs its valid here and in scope
    // this would be a problem bcs when we are freeing the memory
    // they will both will try to free the same memory
    // which leads to memory corruption, a security risk
    let s1 = String::from("hello");
    let s2 = s1;

    // below has an error bcs rust already "moved" the data to s2
    // hence s1 is not USABLE bcs its moved into s2
    // and rust prevent the memory corruption from happening
    // therefore, never use this way of deep copy
    println!("s1 is {}", s2);

    let c1 = String::from("hello");
    let c2 = c1.clone();

    // example of clone
    // with clone, the memory is being removed and we still
    // can access c1 unlike using the move method
    // c1 is accessible and we clone c1 into c2
    println!("c1 is {}, c2 is {}", c1, c2);
}

// cant do stuff with s here bcs it is not valid and not in scope (luar function)