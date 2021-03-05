use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(0, 100);

    loop {
        let mut guess = String::new();

        println!("Guess a number: ");
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // shadowing
        let guess: u32 = match guess.trim().parse() {
            Ok(nun) => nun,
            Err(_) => continue
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
            },
            Ordering::Greater => {
                println!("Too big");
            },
            Ordering::Equal => {
                println!("Correct!");
                break;
            },
        }

        println!("You guessed: {}\n\n", guess);
    }
}
