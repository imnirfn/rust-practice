
#[derive(Debug)]
enum MyState {
    Selangor,
    Johor,
    Kedah,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(MyState),
}

fn main() {
    let penny = Coin::Quarter(MyState::Johor);
    let result = value_in_cents(penny);
    println!("Result is: {}", result);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:#?}", state);
            25
        },
    }
}
