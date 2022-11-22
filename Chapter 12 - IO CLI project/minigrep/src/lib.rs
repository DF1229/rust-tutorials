use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        print!("{line}\n");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line.trim());
        }
    }

    results
}

#[cfg(test)]
mod query_tests {
    use super::*;

    const CONTENTS: &str = "\
        Rust:
        safe, fast, productive.
        Pick three.";

    #[test]
    fn no_result() {
        let query = "largewordthatisnotinthecontent";

        assert_eq!(Vec::<&str>::new(), search(query, CONTENTS));
    }

    #[test]
    fn one_result() {
        let query = "duct";

        assert_eq!(vec!["safe, fast, productive."], search(query, CONTENTS));
    }

    #[test]
    fn two_results() {
        let query = "i";

        assert_eq!(vec!["safe, fast, productive.", "Pick three."], search(query, CONTENTS));
    }
    
}
