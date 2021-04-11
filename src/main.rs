use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    // First argument is the call to the program
    // Second argument is the needle
    // Third argument is the haystack (file name)
    if args.len() < 3 {
        println!("Please provide both a needle and a haystack");
        return;
    }

    let config = parse_config(&args);

    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong when reading the file");


    println!("Searching for {} in text:\n{}.", config.query, contents);
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}