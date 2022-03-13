// TOODO
// 1. read cli arguments [x]
// 2. read a file [x]
//
// refactor code problem
// 1. should make a seperate function to do certain task []
//      -> should seperate logic process into lib.rs
//      -> transfer every logic code into lib.rs
//      -> import lib.rs into main
// 2. groups the use variable so can be easy to handle [x]
//      -> make a function that extracts argument parser
//      -> implement struct to groups the config instead of tuple
//      -> implement a config constructor
// 3. properly handles error to be more specific [x]
//      -> use panic! macro (but this reveals more error log than needed)
//      -> using Result for more controlled error log (Err, Ok)
// 4. 

use std::env;
use std::process;

// importing lib.rs
use minigrep::Config;

fn main() {
    // reading cli arguments
    let args: Vec<String> = env::args().collect();

    // parsing config variable into main
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }

}


