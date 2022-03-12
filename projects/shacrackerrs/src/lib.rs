//  TODO
// 1. verify user input its sha1 [x]
// 2. use sha1 crate []
//
//


use std::fs;
use std::io::{
    BufRead,
    BufReader
};
use std::error::Error;

const SHA1_HEX_STRING_LENGTH: usize = 40;

pub struct Config {
    pub hash: String,
    pub wrdlst: String
}

impl Config {
    pub fn parse_args(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("
                Usage:
                shacrackkerrs <hash> <wordlists.txt>
                ");
        }

        let hash = args[1].clone();
        let wrdlst = args[2].clone();
        
        // checking valid sha1 hash
        if hash.len() != SHA1_HEX_STRING_LENGTH {
            return Err("not a valid sha1 hash")
        }

        Ok(Config {hash, wrdlst})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // read file
    let file = fs::File::open(config.wrdlst)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?.trim().to_string();
        println!("{}", line);
    }

    Ok(())
}
