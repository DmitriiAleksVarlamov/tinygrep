use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub query: String,
    pub file_name: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static String> {
        if args.len() < 3 {
            eprintln!("Not enough arguments. You should give atleast 2");
        }
        let query = args[1].clone();
        let file_name = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_name,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_name)?; // .expect("Unable to read file");

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    // let file = fs::File::open(file_name);}
    for line in results {
        println!("With text:\n{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut vec: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            vec.push(line.trim());
        }
    }

    vec
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut vec: Vec<&str> = Vec::new();
    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            vec.push(line.trim());
        }
    }

    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_rest() {
        let query = "duct";
        let content = "\
        Rust:
        safe, fast, productive.
        Pick three.
        Duct to";

        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }

    #[test]
    fn search_case_insensitive_test() {
        let query = "rUsT";
        let content = "\
        Rust:
        safe, fast, productive.
        trust me.";

        assert_eq!(
            vec!["Rust:", "trust me."],
            search_case_insensitive(query, content)
        );
    }
}
