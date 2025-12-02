use std::env;
use aoc2025::*;

fn main() {
    let mut args= env::args(); args.next();
    let (day, in_file) = (args.next().expect("Arg 1 is day"), args.next().expect("Arg 2 is file name"));
    let input = read(in_file);
    let res = match day.as_str() {
        "1a" => d1a, "1b" => d1b,
        "2a" => d2a, "2b" => d2b,
        _ => panic!("Unknown day")
    };
    println!("{}", res(input));
}