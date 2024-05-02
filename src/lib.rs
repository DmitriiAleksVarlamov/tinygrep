use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_name: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static String> {
        if args.len() < 3 {
            panic!("Not enough arguments. You should give atleast 2");
        }
        let query = args[1].clone();
        let file_name = args[2].clone();

        Ok(Config { query, file_name })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_name)?; // .expect("Unable to read file");
                                                          // let file = fs::File::open(file_name);}
    for line in search(&config.query, &contents) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_rest() {
        let query = "duct";
        let content = "\
        Rust:
        safe, fast, productive.
        Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }
}
