
use std::collections::HashMap;

use regex::Regex;

fn parsed1(input: Vec<String>) -> (Vec<i32>, Vec<i32>) {
    let input: Vec<(i32, i32)> = input.into_iter().map(|l|{
        let l:  Vec<i32> = l.split_ascii_whitespace().map(|n|n.parse().unwrap()).collect();
        (l[0], l[1])
    }).collect();
    input.into_iter().unzip()
}

pub fn d1a(input: Vec<String>) -> String {
    let (mut l1, mut l2) = parsed1(input);
    l1.sort_unstable();
    l2.sort_unstable();
    let res: i32 = l1.into_iter().zip(l2).map(|(a,b)|(a-b).abs()).sum();
    res.to_string()
}

pub fn d1b(input: Vec<String>) -> String {
    let (l1, l2) = parsed1(input);
    let c: HashMap<i32, i32> = HashMap::new();
    let c = l2.into_iter().fold(c, |mut acc,n| {
        *acc.entry(n).or_default()+=1; acc 
    });
    let s: i32 = l1.iter().map(|n| n * c.get(n).unwrap_or(&0)).sum();
    s.to_string()
}

fn d2_is_safe(levels: &Vec<i32>) -> bool {
    fn is_diff_ok(first_diff: i32, cur_diff: i32) -> bool {
        let ca = cur_diff.abs();
        ca>=1 && ca<=3 && first_diff.signum() == cur_diff.signum()
    }
    let first_diff = levels[1]-levels[0];
    for i in 1..levels.len() {
        let cur_diff = levels[i] - levels[i-1];
        if !is_diff_ok(first_diff, cur_diff) {
            return false
        }
    }
    true
}

fn _s2vi(input: &str) -> Vec<i32> {
    input.split_ascii_whitespace().map(|n| n.parse().unwrap()).collect()
}

pub fn d2a(input: Vec<String>) -> String {
    input.into_iter().filter(|l| d2_is_safe(&_s2vi(l))).count().to_string()
}

pub fn d2b(input: Vec<String>) -> String {
    input.into_iter().filter(|l| {
        let level = _s2vi(l);
        if d2_is_safe(&level) {
            return true
        }
        for try_remove in 0..level.len() {
            let mut levelclone = level.clone();
            levelclone.remove(try_remove);
            if d2_is_safe(&levelclone) {
                return true
            }
        }
        false
    }).count().to_string()
}

pub fn d3a(input: Vec<String>) -> String {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(&input.join("")).map(|c|c.extract())
        .map(|(_, [i,j])| i.parse::<i32>().unwrap() * j.parse::<i32>().unwrap() )
        .sum::<i32>().to_string()
}


pub fn d3b(input: Vec<String>) -> String {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let mut enable = true;
    let mut ans = 0;
    for c in re.captures_iter(&input.join("")) {
        let c0 = c.get(0).unwrap().as_str();
        if c0 == r"do()" {
            enable = true;
        } else if c0 == r"don't()" {
            enable = false
        } else if enable {
            let (i,j) = (c.get(1).unwrap(), c.get(2).unwrap());
            ans += i.as_str().parse::<i32>().unwrap() * j.as_str().parse::<i32>().unwrap();
        }
    }

    ans.to_string()
}

fn d4find(input: &[String], mut y: usize, mut x: usize, dy: i32, dx: i32, tofind: &str) -> i32 {
    let h = input.len();
    let w = input[0].len();
    let end_y = y as i32 + dy*tofind.len() as i32;
    if end_y < -1 || end_y > h as i32 { return 0 }  
    let end_x = x as i32 + dx*tofind.len() as i32;
    if end_x < -1 || end_x > w as i32 { return 0 }
    let mut tofind = tofind.as_bytes().iter().peekable();
    while tofind.peek().is_some_and(|&&ch|ch==input[y].as_bytes()[x]) {
        tofind.next();
        y=(y as i32 + dy) as usize;
        x=(x as i32 + dx) as usize;
    }
    if tofind.peek().is_none() { 1 } else {0}
}

pub fn d4a(input: Vec<String>) -> String {
    let mut ans=0;
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            for dy in -1..=1 {
                for dx in -1..=1 {
                    ans += d4find(&input, y, x, dy, dx, "XMAS");
                }
            }
        }
    }
    ans.to_string()
}

pub fn d4b(input: Vec<String>) -> String {
    let mut ans = 0;
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            for w in ["MAS", "SAM"] {
                if d4find(&input, y, x, 1, 1, w)>0 {
                    // flip vert or horz
                    if d4find(&input, y, x+2, 1, -1, w)>0 ||
                       d4find(&input, y+2, x, -1, 1, w)>0
                    {
                        ans+=1;
                    }
                }
            }
        }
    }
    ans.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = d1(vec!["a".to_string()]);
        assert_eq!(result, "4".to_string());
    }
}
