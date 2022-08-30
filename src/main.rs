use std::{env, fs};

fn main() {
    // env::args() gets an iterator of the command line arguments passed to it and collecet()
    // converts it into a collection that can be stored in a vector.
    let args: Vec<String> = env::args().collect();

    let config_params = parse_config(&args);


    println!("Query string: '{}'\nFile path: '{}'", config_params.query, config_params.file_path);

    let contents = fs::read_to_string(config_params.file_path).expect("Should have been able to read this file.");

    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    // The clone method below "clones" the string in args so the Config struct can take ownership
    // of the string. This is a less "optimal" way but is much more simple. Avoids having to deal
    // with ownership and lifetimes.
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config {
        query,
        file_path,
    }
}
