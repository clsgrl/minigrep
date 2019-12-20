use std::fs;
use std::error::Error;

pub struct Config {
    pub query : String,
    pub filename : String,
    pub case_sensitive : bool
}

impl Config {
    pub fn new(args : &[String]) -> Result<Config, &'static str> {

        if args.len() < 3 {
            panic!("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config : Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)
        .expect("error reading the file");

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}


//#![allow(unused_variables)]
pub fn search<'a>(query : &str, contents : &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
}
