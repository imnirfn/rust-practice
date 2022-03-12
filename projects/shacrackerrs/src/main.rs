
use std::env;
use std::process;

// imports logic from lib.rs
use shacrackerrs::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::parse_args(&args).unwrap_or_else(|err| {
        println!("problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("{} {}", config.hash, config.wrdlst);

    // handling error in main
    if let Err(e) = shacrackerrs::run(config) {
        println!("application error: {}", e);
        process::exit(1);
    }

}

