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

    println!("With text:\n{contents}");

    Ok(())
}
