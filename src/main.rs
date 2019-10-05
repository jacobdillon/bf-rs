use bf::run;
use std::env;

fn main() {
    let input = env::args().nth(1).unwrap_or(String::from(""));
    let args = env::args().skip(2).fold(String::from(""), |a, b| a + " " + &b); //joins the arguments by a space

    println!("{}", run(input, args.trim().to_string()));
}
