
use std::{cmp::Ordering, collections::{HashMap, HashSet}};

use rayon::iter::{IntoParallelIterator, ParallelIterator};
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

fn parsed5(input: Vec<String>) -> (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>) {
    let mut edges: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut updates = vec![];
    let mut sect1 = true;
    for line in input {
        if line.is_empty() {
            sect1 = false;
            continue;
        }
        if sect1 {
            let from_to: Vec<i32> = line.split('|').map(|s|s.parse().unwrap()).collect();
            edges.entry(from_to[0]).or_default().insert(from_to[1]);
        } else {
            let ns: Vec<i32> = line.split(',').map(|s|s.parse().unwrap()).collect();
            updates.push(ns);
        }
    }
    (edges, updates)

}

pub fn d5a(input: Vec<String>) -> String {
    let (edges, updates) = parsed5(input);
    let mut ans = 0;
    for update in updates {
        let mut is_valid = true;
        for i in 0..update.len()-1 {
            for j in i+1..update.len() {
                let from = update[i];
                let to = update[j];
                if let Some(tos) = edges.get(&from) {
                    if !tos.contains(&to) {
                        is_valid = false;
                        break
                    }
                }
                if let Some(tos) = edges.get(&to) {
                    if tos.contains(&from) {
                        is_valid = false;
                        break
                    }
                }
            }
            if !is_valid { break }
        }
        if is_valid {
            let mid = update.len()/2;
            ans+=update[mid];
        }
    }
    ans.to_string()
}

pub fn d5b(input: Vec<String>) -> String {
    let (edges, updates) = parsed5(input);
    let mut ans = 0;
    for update in &updates {
        let mut sortedupd = update.clone();
        sortedupd.sort_unstable_by(|a,b| {
            if let Some(tos) = edges.get(a) {
                if tos.contains(b) {
                    return Ordering::Less
                }
            }
            if let Some(tos) = edges.get(b) {
                if tos.contains(a) {
                    return Ordering::Greater
                }
            }
            panic!("Can't find both {a} {b} {updates:?}");
        });
        if update.iter().zip(&sortedupd).any(|(a,b)|*a!=*b) {
            ans += sortedupd[sortedupd.len()/2];
        }
    }
    ans.to_string()
}

fn d6guard(input: &[Vec<u8>]) -> (usize, usize) {
    for (y, row) in input.iter().enumerate() {
        for (x, ch) in row.iter().enumerate() {
            if *ch == '^' as u8 {
                return (y,x)
            }
        }
    }
    unreachable!("Guard not found");
}

fn d6runguard(input: &[Vec<u8>], oy: usize, ox: usize) -> i32 {
    let (mut y, mut x) = d6guard(&input);
    let dirs = [(-1isize,0isize),(0,1),(1,0),(0,-1)];
    let mut dir = 0;
    let mut ans = 1;
    let h = input.len() as isize;
    let w = input[0].len() as isize;

    const UNVISITED: usize = 7;
    let mut visited = vec![vec![UNVISITED; w as usize]; h as usize];
    visited[y][x] = dir;
    loop {
        let ny = y as isize + dirs[dir].0;
        let nx = x as isize + dirs[dir].1;
        if ny<0 || ny>=h || nx < 0 || nx >= w {
            break
        }
        let ny = ny as usize;
        let nx = nx as usize;
        if ny==oy && nx==ox ||
           input[ny][nx] == '#' as u8 {
            dir = (dir + 1)%dirs.len();
            continue
        }
        if visited[ny][nx]==dir {
            return -1    // loop found
        }
        if visited[ny][nx] == UNVISITED {
            ans+=1;
            visited[ny][nx]=dir;
        }
        y = ny;
        x = nx;
    }
    ans
}

pub fn d6a(input: Vec<String>) -> String {
    let input: Vec<_> = input.into_iter().map(|l|l.as_bytes().to_vec()).collect();
    d6runguard(&input, usize::MAX, usize::MAX).to_string()
}

pub fn d6b(input: Vec<String>) -> String {
    let input: Vec<_> = input.into_iter().map(|l|l.as_bytes().to_vec()).collect();
    let ans: i32 = (0..input.len()).into_par_iter().map(|y| {
        (0..input[y].len()).into_par_iter().map(|x| {
            if d6runguard(&input, y, x) == -1 {
                1
            }
            else { 0 }
        }).sum::<i32>()
    }).sum();
    ans.to_string()
}

fn parsed7(input: Vec<String>) -> Vec<(i64, Vec<i64>)> {
    input.into_iter().map(|l| {
        let mut fs = l.split(&[':', ' ']);
        (fs.next().unwrap().parse::<i64>().unwrap(), 
        fs.skip(1).map(|c|c.parse::<i64>().unwrap()).collect())
    }).collect()
}

fn d7canequal(target: i64, nums: &[i64], idx: usize, total: i64, can_concat: bool) -> bool {
    if idx >= nums.len() {
        return total == target
    }
    if total > target {
        return false
    }
    d7canequal(target, nums, idx+1, total+nums[idx], can_concat) ||
    d7canequal(target, nums, idx+1, (if idx==0 {1 } else {total})*nums[idx], can_concat) ||
    can_concat && d7canequal(target, nums, idx+1, total*10i64.pow(nums[idx].to_string().len() as u32) + nums[idx], can_concat)
}

pub fn d7a(input: Vec<String>) -> String {
    let input = parsed7(input);
    let ans: i64 = input.into_par_iter().filter_map(|i| {
        if d7canequal(i.0, &i.1, 0, 0, false) {
            Some(i.0)
        }
        else { None }
    }).sum();
    ans.to_string()
}

pub fn d7b(input: Vec<String>) -> String {

    let input = parsed7(input);
    let ans: i64 = input.into_par_iter().filter_map(|i| {
        if d7canequal(i.0, &i.1, 0, 0, true) {
            Some(i.0)
        }
        else { None }
    }).sum();
    ans.to_string()
}

fn parsed8(input: Vec<String>) -> HashMap<u8, Vec<(isize, isize)>> {
    let mut ans: HashMap<u8, Vec<(isize, isize)>>  = HashMap::new();
    for (y, row) in input.into_iter().enumerate() {
        for (x, &ch) in row.as_bytes().into_iter().enumerate() {
            if ch != '.' as u8 {
                ans.entry(ch).or_default().push((y as isize,x as isize));
            }
        }
    }
    ans
}

fn d8caltanti(input: Vec<String>, resonant: bool) -> HashSet<(isize, isize)> {
    let h= input.len() as isize;
    let w = input[0].len() as isize;
    let input = parsed8(input);
    let antilocs: HashSet<_> = input.into_par_iter().flat_map(|(_,locs)| {
        let mut antins = Vec::with_capacity(locs.len().pow(2));
        for n1 in &locs {
            for n2 in &locs {
                if n1 != n2 {
                    let dy = n2.0 - n1.0;
                    let dx = n2.1 - n1.1;
                    let mut antiy = n2.0;
                    let mut antix = n2.1;
                    if resonant {
                        antins.push((antiy, antix));
                    }
                    loop {
                        antiy += dy;
                        antix += dx;
                        if antiy>=0 && antiy < h && antix>=0 && antix < w {
                            antins.push((antiy, antix));
                            if !resonant { break }
                        }
                        else {
                            break
                        }
                    }
                }
            }
        }
        antins
    }).collect();
    antilocs
}

pub fn d8a(input: Vec<String>) -> String {
    d8caltanti(input, false).len().to_string()
}

pub fn d8b(input: Vec<String>) -> String {
    d8caltanti(input, true).len().to_string()
}

#[derive(Debug)]
struct D9defrag {
    map: Vec<u8>,
    curidx: isize,
    curidxexpand: isize,
    tomoveidx: isize,
}

impl D9defrag {
    fn new(input: &str) -> D9defrag {
        let chars: Vec<_> = input.as_bytes().iter().map(|ch| ch-'0' as u8).collect();
        D9defrag {
            curidx: 0,
            curidxexpand: 0,
            tomoveidx: chars.len() as isize - 2 + chars.len() as isize%2,
            map: chars,
        }
    }

    fn is_space(idx: isize) -> bool {
        idx%2 != 0
    }

    fn checksum(fileno: isize, pos: isize, size: u8) -> i64 {
        let mut ans = 0;
        // fileno*pos + fileno*(pos+1) + fileno*(pos+2)
        // fileno*pos*(0+1+2+...+ size-1)
        for i in pos..pos+size as isize {
            ans+=fileno*i;
        }
        ans as i64
    }

    fn fillspace(&mut self) -> Vec<(isize, isize, u8)> {  // (origpos, newpos, size)
        let mut ans = Vec::new();
        let mut avail_space = self.map[self.curidx as usize];
        while self.curidx < self.tomoveidx {
            let tomove = avail_space.min(self.map[self.tomoveidx as usize]);
            self.map[self.tomoveidx as usize] -= tomove;
            avail_space -= tomove;
            ans.push((self.tomoveidx, self.curidxexpand, tomove));
            self.curidxexpand += tomove as isize;
            if self.map[self.tomoveidx as usize]==0 {
                self.tomoveidx-=2;
            }
            if avail_space == 0 {
                break
            }
        }
        ans
    }

    fn go(mut self) -> i64 {
        let mut ans = 0;
        while self.curidx <= self.tomoveidx {
            if D9defrag::is_space(self.curidx) {
                let moved = self.fillspace();
                for (origpos, newpos, size) in &moved {
                    ans += D9defrag::checksum(*origpos/2, *newpos, *size);
                }
            }
            else {
                ans += D9defrag::checksum(self.curidx/2, self.curidxexpand, self.map[self.curidx as usize]);
                self.curidxexpand += self.map[self.curidx as usize] as isize;
            }
            self.curidx+=1;
        }
        ans
    }
}


pub fn d9a(input: Vec<String>) -> String {
    let input = D9defrag::new(&input[0]);
    input.go().to_string()
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
