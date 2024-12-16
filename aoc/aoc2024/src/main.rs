use std::env;
use aoc2024::*;

fn main() {
    let mut args= env::args(); args.next();
    let (day, in_file) = (args.next().expect("Arg 1 is day"), args.next().expect("Arg 2 is file name"));
    let input = read(in_file);
    let res = match day.as_str() {
        "1a" => d1a,
        "1b" => d1b,
        "2a" => d2a,
        "2b" => d2b,
        "3a" => d3a,
        "3b" => d3b,
        "4a" => d4a,
        "4b" => d4b,
        "5a" => d5a, "5b" => d5b,
        "6a" => d6a, "6b" => d6b,
        "7a" => d7a, "7b" => d7b,
        "8a" => d8a, "8b" => d8b,
        "9a" => d9a, "9b" => d9b,
        "10a" => d10a, "10b" => d10b,
        "11a" => d11a, "11b" => d11b,
        "12a" => d12a, "12b" => d12b,
        "13a" => d13a, "13b" => d13b,
        "14a" => d14a, "14b" => d14b,
        "15a" => d15a, "15b" => d15b,
        "16a" => d16a,  
        _ => panic!("Unknown day")
    };
    println!("{}", res(input));
}