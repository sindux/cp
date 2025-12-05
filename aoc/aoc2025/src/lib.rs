use std::{cmp::{Ordering, Reverse}, collections::{BinaryHeap, HashMap, HashSet, VecDeque}, fmt::Debug, fs, io::{self, BufRead}, iter::FlatMap, str::FromStr};


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

#[allow(dead_code)]
fn ss2vvc(grid: &[&str]) -> Vec<Vec<char>> {
    grid.iter().map(|l|l.chars().collect())
        .collect()
}

#[allow(dead_code)]
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

#[allow(dead_code)]
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

fn d2a_invalid(start: i64, end: i64) -> Vec<i64> {
    let mut ans = vec![];
    for i in start..=end {
        let mut j = i;
        let mut digit_count = 0;
        while j>0 {
            digit_count += 1;
            j /=10;
        }
        if digit_count%2 != 0 {
            continue;
        }
        j = i;

        let mut right = 0;
        let mut mult=1;
        for _ in 0..(digit_count/2) {
            right = mult * (j%10) + right;
            mult *= 10;
            j/=10;
        }
        let mut left = 0;
        mult = 1;
        for _ in 0..(digit_count/2) {
            left = mult * (j%10) + left;
            mult *= 10;
            j/=10;
        }
        if left==right {
            ans.push(i);
        }
    }

    ans
}

fn d2b_invalid(start: i64, end: i64) -> Vec<i64> {
   let mut ans = vec![];
    for i in start..=end {
        let mut j = i;
        let mut digit_count = 0;
        while j>0 {
            digit_count += 1;
            j /=10;
        }
        for l in 1..=(digit_count/2) {
            if digit_count%l != 0 {
                continue;
            }
            j = i;

            let mut prev = -1;
            let mut allgood=true;
            for _ in 0..(digit_count/l) {
                let mut cur = 0;
                let mut mult = 1;
                for _ in 0..l {
                    cur = mult * (j%10) + cur;
                    mult *=10;
                    j/=10;
                }
                if prev != -1 && prev != cur {
                    allgood = false;
                    break;
                }
                prev = cur;
            }
            if allgood {
                ans.push(i);
                break
            }
        }
    }

    ans
}

pub fn d2a(input: Vec<String>) -> String {
    assert!(input.len()==1);
    input[0].split(',').map(|r| {
        let mut rr = r.split('-');
        [rr.next().unwrap().parse::<i64>().unwrap(), rr.next().unwrap().parse().unwrap()]
    }).flat_map(|r| d2a_invalid(r[0], r[1])).sum::<i64>().to_string()
}

pub fn d2b(input: Vec<String>) -> String {
    assert!(input.len()==1);
    input[0].split(',').map(|r| {
        let mut rr = r.split('-');
        [rr.next().unwrap().parse::<i64>().unwrap(), rr.next().unwrap().parse().unwrap()]
    }).flat_map(|r| d2b_invalid(r[0], r[1])).sum::<i64>().to_string()
}

pub fn d3a(input: Vec<String>) -> String {
    let mut ans = 0i32;
    for pack in input {
        let mut thisrow = 0;
        let pack = pack.as_bytes();
        for (i, ch) in pack.iter().enumerate() {
            for (j, ch2) in pack.iter().enumerate() {
                if i>=j {
                    continue;
                }
                thisrow = thisrow.max(
                    (ch - b'0') *10 + ch2 - b'0'
                );
            }
        }
        ans+=thisrow as i32;
    }

    ans.to_string()
}

fn d3b_go(pack: &[u8], idx: usize, pos: usize, memo: &mut [Vec<i64>]) -> i64 {
    if idx>=pack.len() {
        return 0;
    }
    // println!("idx: {idx} pos: {pos} len: {}", pack.len());

    if memo[idx][pos]!=0 {
        return memo[idx][pos];
    }
    // len: 5
    // idx: 0 1 2 3 4
    // pos:   3 2 1 0
    if pos == 0 {
        memo[idx][pos] = (pack[idx] as i64).max(d3b_go(pack, idx+1, pos, memo));
        // println!("  1--> {}", memo[idx][pos]);
        return memo[idx][pos];
    }
    if idx + 1 + pos == pack.len() {
        if memo[idx][pos]!=0 {
            return memo[idx][pos];
        }
        memo[idx][pos] = pack[idx] as i64 * 10_i64.pow(pos as u32) + d3b_go(pack, idx+1, pos-1, memo);
        // println!("  2--> {}", memo[idx][pos]);
        return memo[idx][pos];
    }
    memo[idx][pos] = d3b_go(pack, idx+1, pos, memo).max(
        pack[idx] as i64 * 10_i64.pow(pos as u32) + d3b_go(pack, idx+1, pos-1, memo)
    );
    // println!("  3--> {}", memo[idx][pos]);
    memo[idx][pos]
}

pub fn d3b(input: Vec<String>) -> String {
    let mut ans = 0;
    for pack in input {
        let pack = pack.as_bytes();
        let pack = pack.iter().map(|&b| b - b'0').collect::<Vec<u8>>();
        let mut memo = vec![vec![0; 12]; pack.len()];
        ans+=d3b_go(&pack, 0, 11, &mut memo);
    }
    ans.to_string()
}

fn d4_parse(line: Vec<String>) -> Vec<Vec<char>> {
    let l = line[0].len();
    let mut ans = vec![];
    ans.push(vec!['.'; l+2]);
    for l in line {
        let mut row = vec!['.'];
        row.extend(l.chars());
        row.push('.');
        ans.push(row);
    }
    ans.push(vec!['.'; l+2]);
    ans
}

fn d4(input: &[Vec<char>]) -> Vec<(usize, usize)> {
    let mut can_remove = vec![];
    for y in 1..(input.len()-1) as isize {
        for x in 1..(input[0].len()-1) as isize {
            if input[y as usize][x as usize]!='@' {
                continue;
            }
            let mut cnt = 0;
            for dy in -1..=1isize {
                for dx in -1..=1isize {
                    if !(dy == 0 && dx == 0) && input[(y+dy) as usize][(x+dx) as usize]=='@' {
                        cnt +=1;
                    }
                }
            }
            
            if cnt < 4 {
                can_remove.push((y as usize, x as usize));
            }
        }
    }
    can_remove
}

pub fn d4a(input: Vec<String>) -> String {
    let input = d4_parse(input);
    d4(&input).len().to_string()
}

pub fn d4b(input: Vec<String>) -> String {
    let mut ans = 0;
    let mut input = d4_parse(input);
    loop {
        let can_remove = d4(&input);
        ans += can_remove.len();
        for &(y, x) in &can_remove {
            input[y][x] = '.';
        }
        if can_remove.is_empty() {
            break;
        }
    }
    ans.to_string()
}

fn d5a_parse(input: Vec<String>) -> (Vec<i64>, Vec<i64>, Vec<i64>) {
    let mut ranges = vec![];
    let mut queries = vec![];
    let mut which = 0;
    for i in input {
        if i.is_empty() {
            which = 1;
            continue;
        }
        if which == 0 {
            let mut rr = i.split('-');
            let start = rr.next().unwrap().parse::<i64>().unwrap();
            let end = rr.next().unwrap().parse::<i64>().unwrap();
            ranges.push((start, end));
        } else {
            queries.push(i.parse::<i64>().unwrap());
        }
    }
    ranges.sort_unstable();
    (ranges.iter().map(|x|x.0).collect(),
     ranges.iter().map(|x|x.1).collect(),
     queries)
}

pub fn d5a(input: Vec<String>) -> String {
    let (r_from, r_to, queries) = d5a_parse(input);
    let mut ans = 0;
    for q in queries {
        for i in 0..r_from.len() {
            if r_from[i]<=q && q<=r_to[i] {
                ans+=1;
                break;
            }
        }
    }
    ans.to_string()
}

pub fn d5b(input: Vec<String>) -> String {
    // 357907198933888 too low
    let (r_from, r_to, _) = d5a_parse(input);
    let mut ans = 0;
    let mut prev = 0;
    for (from, to) in r_from.into_iter().zip(r_to.into_iter()) {
        let mut add=0;
        if to>=prev {
            add = to - prev.max(from) + 1;
            ans += add;
            prev = prev.max(to) + 1;
        }
    }
    ans.to_string()
}