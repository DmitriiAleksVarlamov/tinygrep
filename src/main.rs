use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();
    dbg!(args);
    println!("Hello, world!");
}
