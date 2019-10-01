use bf::run;
use std::env;

fn main() {
    let input = env::args().nth(1).unwrap_or(String::from(""));
    let args: Vec<String> = env::args().skip(2).collect();

    println!("{}", run(input, args));
}
