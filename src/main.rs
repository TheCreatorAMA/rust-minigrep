use std::env;

fn main() {
    // env::args() gets an iterator of the command line arguments passed to it and collecet()
    // converts it into a collection that can be stored in a vector.
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Query string: '{}'\nFile path: '{}'", query, file_path);
}
