
use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    substring: String,
    filename: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(
        mut args: impl Iterator <Item = String>,
    ) -> Result<Config, &'static str> {
        args.next();

        let substring = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };
        
        if args.next() != None {
            return Err("Incorrect usage, example of correct usage:\n\t'cargo run -- search_string file_path'");
        }

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config{substring, filename, ignore_case})
    }

    pub fn new(substring: String, filename: String, ignore_case: bool) -> Config {
        Config{substring, filename, ignore_case}
    }
}

pub fn run(cfg: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(cfg.filename)?;
    
    let results = if cfg.ignore_case {
        search_case_insensitive(&cfg.substring, &contents)
    } else {
        search(&cfg.substring, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(substring: &str, contents: &'a str) -> Vec<&'a str> { 
    contents.lines().filter(|line| line.contains(substring)).collect()
}

pub fn search_case_insensitive<'a>(substring: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.to_lowercase().contains(&substring.to_lowercase())).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn newtest() {
        let substring = String::from("hello");
        let filename = String::from("five");
        let ignore_case = true;
        let actual = Config::new(substring.clone(), filename.clone(), ignore_case);
        let expected = Config{substring, filename, ignore_case};
        assert_eq!(actual.filename, expected.filename);
        assert_eq!(actual.substring, expected.substring);
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nDuct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nTrust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}