use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVES").is_err();
        
        Ok(Config {query, filename, case_sensitive})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // reading a file
    let data = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &data)
    } else {
        search_case_insensitives(&config.query, &data)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

// TODD
// 1. refactor search funtion using iterators
//

pub fn search<'a>(query: &str, data: &'a str) -> Vec<&'a str> { //tell rust this will be return from a split data string, thats why we are usig explicit lifetime
    let mut results = Vec::new();
    for line in data.lines() {
        // println!("line: {}, data: {}", query, data);
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitives<'a>(query: &str, data: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in data.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let data = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, data));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUSt";
        let data = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitives(query, data));
    }
}

