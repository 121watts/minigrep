use std::error::Error;
use std::fs;

#[derive(Debug, PartialEq)]
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let matches = search(&config.query, &contents);

    for line in matches {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut matches = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            matches.push(line);
        }
    }

    matches
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

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn not_enough_args() {
        let args = [String::new(), String::new()];
        let config = Config::new(&args);
        assert_eq!(config, Err("not enough arguments"))
    }

    #[test]
    fn correct_config() {
        let query = String::from("plerps");
        let filename = String::from("plerps.txt");
        let args = [String::new(), query, filename];

        let test_config = Config {
            query: "plerps".to_owned(),
            filename: "plerps.txt".to_owned(),
        };

        let config = Config::new(&args);

        assert_eq!(Ok(test_config), config);
    }
}
