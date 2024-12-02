use std::{env, fs, io::{self, BufRead}};
use aoc2024::*;

fn read(f: String) -> Vec<String>
{
    let input = io::BufReader::new(fs::File::open(f).expect("Error reading input file"));
    input
        .lines()
        .map(|line| line.expect("Error reading input line"))
        .collect()
}

fn main() {
    let mut args= env::args(); args.next();
    let (day, in_file) = (args.next().expect("Arg 1 is day"), args.next().expect("Arg 2 is file name"));
    let input = read(in_file);
    let res = match day.as_str() {
        "1a" => d1a(input),
        "1b" => d1b(input),
        "2a" => d2a(input),
        _ => "Unknown day".to_string()
    };
    println!("{res}");
}