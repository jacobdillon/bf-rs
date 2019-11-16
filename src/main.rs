use bf::run;
use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let program_src = &args[0];
    let program_args = &args[1..];
    println!("{}", run(program_src, program_args));
}