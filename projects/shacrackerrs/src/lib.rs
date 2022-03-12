use std::fs;
use std::error::Error;

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

        Ok(Config {hash, wrdlst})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // read file
    let contents = fs::read_to_string(config.wrdlst)
        .expect("error");
    println!("wordlist contents: {}", contents);

    Ok(())
}
