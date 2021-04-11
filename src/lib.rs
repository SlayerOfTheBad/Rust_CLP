use std::fs;
use std::error::Error;

pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        // First argument is the call to the program
        // Second argument is the needle
        // Third argument is the haystack (file name)
        if args.len() < 3 {
            return Err("Not enough arguments have been provided");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;

    println!("Searching for {} in text:\n{}.", config.query, contents);

    Ok(())
}