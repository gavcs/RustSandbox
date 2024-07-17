
use std::fs;
use std::error::Error;

pub struct Config {
    substring: String,
    filename: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 3 {
            return Err("Incorrect usage, example of correct usage:\n\t'cargo run -- search_string file_path'");
        }
        let substring = args[1].clone();
        let filename = args[2].clone();
        Ok(Config{substring, filename})
    }

    pub fn new(substring: String, filename: String) -> Config {
        Config{substring, filename}
    }
}

pub fn run(cfg: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(cfg.filename)?;
    println!("{}\n{contents}", cfg.substring);

    Ok(())
}

pub fn search<'a>(substring: &str, contents: &'a str) -> Vec<&'a str> { 
    vec![]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn newtest() {
        let substring = String::from("hello");
        let filename = String::from("five");
        let actual = Config::new(substring.clone(), filename.clone());
        let expected = Config{substring, filename};
        assert_eq!(actual.filename, expected.filename);
        assert_eq!(actual.substring, expected.substring);
    }

    #[test]
    fn runtest() {
        let substring = String::from("name");
        let filename = String::from("poem");
        let con = Config{substring, filename};
        let actual = run(con).unwrap();
    }

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
                            Rust:
                            safe, fast, productive.
                            Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}