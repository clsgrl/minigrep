use std::fs;
use std::error::Error;

pub struct Config {
    pub query : String,
    pub filename : String
}

impl Config {
    pub fn new(mut args : std::env::Args) -> Result<Config, &'static str> {

        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string")
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get the file name")
        };

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
    contents.lines().filter(|line| line.contains(query)).collect()
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
