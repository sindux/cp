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
    let res = match day.as_str() {
        "1" => d1(read(in_file)),
        _ => "Unknown day".to_string()
    };
    println!("{res}");
}