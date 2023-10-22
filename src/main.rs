use std::env;
use std::fs;

fn main() {
    let filepath = env::args().nth(1).expect("no file given");

    let contents = fs::read_to_string(filepath).expect("something went wrong reading the file");

    println!("{}", contents);
}
