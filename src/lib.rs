use std::fs;
use std::error::Error;

pub fn run (config: Config) -> Result<(), Box<dyn Error>> {
    let _contents = fs::read_to_string(config.file_path)?;

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // Handling if there are not enough args
        if args.len() != 3 {
            return Err("Invalid number of arguments, expected 2");
        }
        // The clone method below "clones" the string in args so the Config struct can take ownership
        // of the string. This is a less "optimal" way but is much more simple. Avoids having to deal
        // with ownership and lifetimes.
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }
}
