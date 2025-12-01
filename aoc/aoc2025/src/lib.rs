use std::{cmp::{Ordering, Reverse}, collections::{BinaryHeap, HashMap, HashSet, VecDeque}, fmt::Debug, fs, io::{self, BufRead}, str::FromStr};


pub fn read(f: String) -> Vec<String>
{
    let input = io::BufReader::new(fs::File::open(f).expect("Error reading input file"));
    input
        .lines()
        .map(|line| line.expect("Error reading input line"))
        .collect()
}

#[allow(dead_code)]
fn vs2vvc(grid: Vec<String>) -> Vec<Vec<char>> {
    grid.iter().map(|l|l.chars().collect())
        .collect()
}

fn ss2vvc(grid: &[&str]) -> Vec<Vec<char>> {
    grid.iter().map(|l|l.chars().collect())
        .collect()
}

fn vvc_find(grid: &[Vec<char>], what: char) -> (usize, usize) {
    for (y, row) in grid.iter().enumerate() {
        for (x, ch) in row.iter().enumerate() {
            if *ch == what {
                return (y,x)
            }
        }
    }
    panic!("Cannot find {what} in {}", vvc2str(grid));
}

#[allow(dead_code)]
fn vvc2str(grid: &[Vec<char>]) -> String {
    grid.iter().map(|l|l.iter().collect::<String>())
        .collect::<Vec<_>>().join("\n")

}

fn ss2vs(sliceofstr: &[&str]) -> Vec<String> {
    sliceofstr.iter().map(|&s|s.to_owned()).collect()
}

pub fn d1a(input: Vec<String>) -> String {
    input.into_iter().scan(50, |pos, i| {
        let i = i.as_bytes();
        let sign = if i[0]==b'L' { -1 } else { 1 };
        
        let steps = i[1..].iter().fold(0, |acc, &b| acc*10 + (b - b'0') as isize);
        *pos += sign * steps;
        *pos %= 100;
        Some(*pos)
    }).filter(|&pos|pos==0).count().to_string()
}

pub fn d1b(input: Vec<String>) -> String {
    // 5630 too small
    input.into_iter().fold((50, 0), |(mut pos, mut cnt), i| {
        let i = i.as_bytes();
        let sign = if i[0]==b'L' { -1 } else { 1 };
        
        let mut steps = i[1..].iter().fold(0, |acc, &b| acc*10 + (b - b'0') as isize);

        cnt += steps / 100;
        steps %= 100;

        let newpos = pos + sign * steps;
        if newpos.signum() + pos.signum() == 0  // crossed zero (flipping direction)
            || newpos.abs()>100 //crossed 0 (same dir)
            || newpos.abs()%100 == 0 {// stopped at 0
            cnt += 1;
        } 

        pos = newpos%100;
        (pos, cnt)
    }).1.to_string()
}
