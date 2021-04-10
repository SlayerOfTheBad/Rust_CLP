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

    let query = &args[1];
    let filename = &args[2];

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong when reading the file");


    println!("Searching for {} in text \n{}.", query, contents);
}
