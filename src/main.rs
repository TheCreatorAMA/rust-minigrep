use minigrep::Config;
use std::{env, process};

fn main() {
    // env::args() gets an iterator of the command line arguments passed to it and collecet()
    // converts it into a collection that can be stored in a vector.
    let args: Vec<String> = env::args().collect();

    let config_params = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // if let does the same thing as the unwrap or else above, if there is an error, it does the
    // work in the brackets below, if no error occurs it returns what is in Result<>
    if let Err(e) = minigrep::run(config_params) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
