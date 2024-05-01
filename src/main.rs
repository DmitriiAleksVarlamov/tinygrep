use std::env;
use std::process;
use tinygrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    println!("query = {}, file_name = {}", config.query, config.file_name);

    if let Err(e) = tinygrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    };
}
