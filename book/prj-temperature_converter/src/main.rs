use std::io;

fn main() {
    // 1. prompt two options (C -> F, F -> C)
    // 2. request an input
    // 3. make calculations
    // 4. prints out the results

    loop {
        let mut input = String::new();
        let mut temp = String::new();
        let mut result: i32 = 0;

        println!("Choose converter\n1.Celcius to Farenheit\n2.Farenheit to Celcius\n3.Exit");
        io::stdin().read_line(&mut input).expect("Failed to read line");

        println!("Type a number");
        io::stdin().read_line(&mut temp).expect("Failed to read line");

        // parsing into integer
        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a proper input!");
                continue;
            }
        };
        let temp: i32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a proper input!");
                continue;
            }
        };

        if input == 1 {
            result = celcius_to_farenheit(temp);
        } else if input == 2 {
            result = farenheit_to_celcius(temp);
        } else {
            println!("GoodBye!");
            break;
        }
        println!("Hello, {}!", result);
    }
}

fn celcius_to_farenheit(x: i32) -> i32 {
    // (0°C × 9/5) + 32 = 32°F
    (x * 9 / 5) + 32
}

fn farenheit_to_celcius(x: i32) -> i32 {
    // (10°F − 32) × 5/9 = -12.22°C
    (x - 32) * 5 / 9
}