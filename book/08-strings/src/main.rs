fn main() {
    // mutable srting
    let mut s = String::new();
    let data = "initial contents";

    let s = data.to_string();

    println!("{}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    combine_strings();
		
}

fn combine_strings() {
   
		let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // let s = s1 + "-" + &s2 + "-" + &s3;
    let s = format!("{}-{}-{}", s1, s2, s3);

    println!("s: {}", s);

    // iterating over strings
    let hello = "Здравствуйте";
    
    for c in hello.chars() {
        println!("char: {}", c);
    }

}
