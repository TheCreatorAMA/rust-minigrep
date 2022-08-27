use std::{env, fs};

fn main() {
    // env::args() gets an iterator of the command line arguments passed to it and collecet()
    // converts it into a collection that can be stored in a vector.
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Query string: '{}'\nFile path: '{}'", query, file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read this file.");

    println!("With text:\n{}", contents);
}
