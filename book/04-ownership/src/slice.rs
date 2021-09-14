//  [TASK]
//  write a function that takes a string 
//  and returns the first word it finds in that string
//  If the function doesn’t find a space in the string,
//  the whole string must be one word,
//  so the entire string should be returned.

fn main() {
    let s = String::from("hello world");

    // slice examples
    // let a = &s[0..5];
    // println!("{}", a);

    // let string_literal = "holla world";
    // let literal_slice = &string_literal[0..5];
    // println!("{}", literal_slice);

    let word = first_word(&string_literal);
    println!("First word is: {}", word);

    let word_two = second_word(&string_literal);
    println!("Second word is: {}", word_two);

}

// The type that signifies “string slice” is written as &str
fn first_word(s: &str) -> &str {
    // converts into bytes so we can loop
    // thru each element
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { // search bytes that represents space
            return &s[0..i]; // returns until found space bytes
        }
    }

    &s[..] // returns the entire string
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut first_index: usize = 0;
    let mut found_first = false;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' && !found_first {
            first_index = i + 1;
            found_first = true;
        } else if item == b' ' && found_first {
            return &s[first_index..i];
        }
    }

    ""
}


