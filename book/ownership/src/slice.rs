//  [TASK]
//  write a function that takes a string 
//  and returns the first word it finds in that string
//  If the function doesn’t find a space in the string,
//  the whole string must be one word,
//  so the entire string should be returned.

fn main() {
    let s = String::from("hello world");

    // slice examples
    let a = &s[0..5];
    println!("{}", a);

    let string_literal = "holla world";
    let literal_slice = &string_literal[0..5];
    println!("{}", literal_slice);

    let word = first_word(&string_literal);
    println!("First word is: {}", word);

}

// The type that signifies “string slice” is written as &str
fn first_word(s: &str) -> &str {
    // converts into bytes so we can loop
    // thru each element
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { // search bytes that represents space
            println!("in here");
            return &s[0..i]; // returns until found space bytes
        }
    }

    &s[..] // returns the entire string
}
