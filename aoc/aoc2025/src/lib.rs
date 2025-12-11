use std::{cmp::{Ordering, Reverse}, collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque}, fmt::Debug, fs, io::{self, BufRead}, iter::FlatMap, str::FromStr};
use regex::Regex;   
mod ds;
use ds::UnionFind;

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
fn vvu2str(grid: &[Vec<u8>]) -> String {
    grid.iter().map(|l|l.iter().map(|&c|(c+b'0') as char).collect::<String>())
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

fn d6a_parse(input: Vec<String>) -> (Vec<Vec<i64>>, Vec<char>) {
    let nums: Vec<_> = input.iter().take(input.len()-1).map(|line| {
        line.split_ascii_whitespace().map(|num| num.parse::<i64>().unwrap()).collect()
    }).collect();
    let chars: Vec<_> = input.last().unwrap().split_ascii_whitespace().map(|c| c.chars().next().unwrap()).collect();
    (nums, chars)
}

pub fn d6a(input: Vec<String>) -> String {
    let (nums, chars) = d6a_parse(input);
    chars.into_iter().enumerate().fold(0, |acc, (i, c)| {
        acc + nums.iter().skip(1).fold(nums[0][i], |colacc, nums| {
            match c {
                '+' => colacc + nums[i],
                '*' => colacc * nums[i],
                _ => unreachable!()
            }
        })
    }).to_string()
}

fn d6b_parse(input: Vec<String>) -> (Vec<Vec<i64>>, Vec<char>) {
    let mut idx = 0;
    let mut ops:Vec<_> = input.last().unwrap().chars().collect();
    ops.push(' ');
    let mut nums = vec![];

    let input = input.iter().take(input.len()-1).map(|line| line.as_bytes()).collect::<Vec<_>>();

    while idx < ops.len() {
        let startidx = idx;
        idx += 1;
        while idx < ops.len() && ops[idx].is_whitespace() {
            idx += 1;
        }
        let mut colnums = vec![];
        for x in startidx..idx-1 {
            let mut num = String::with_capacity(input.len());
            for y in 0..input.len() {
                num.push(input[y][x] as char);
            }
            colnums.push(num.trim().parse().unwrap())
        }
        nums.push(colnums);
    }
    
    (nums, ops.into_iter().filter(|&c|!c.is_whitespace()).collect())
}

pub fn d6b(input: Vec<String>) -> String {
    let (nums, ops) = d6b_parse(input);

    ops.into_iter().zip(nums.into_iter()).fold(0i64, |acc, (op, nums)| {
        let val = match op {
            '+' => nums.iter().sum::<i64>(),
            '*' => nums.iter().product::<i64>(),
            _ => unreachable!()
        };
        acc + val
    }).to_string()
}

pub fn d7a(input: Vec<String>) -> String {
    let w = input[0].len();
    let input = vs2vvc(input);
    let mut beams = HashSet::new();
    beams.insert(input[0].iter().position(|n|n==&'S').unwrap());
    let mut ans=0;
    for y in &input[1..] {
        let mut newbeams = HashSet::new();
        for &x in &beams {
            if y[x]=='^' {
                ans+=1;
                if x>0 {
                    newbeams.insert(x-1);
                }
                if x+1<w {
                    newbeams.insert(x+1);
                }
            } else {
                newbeams.insert(x);
            }
        }
        beams = newbeams;
    }
    ans.to_string()
}

pub fn d7b(input: Vec<String>) -> String {
    let w = input[0].len();
    let input = vs2vvc(input);
    let mut beams = HashMap::new();
    beams.insert(input[0].iter().position(|n|n==&'S').unwrap(), 1);
    for y in &input[1..] {
        let mut newbeams = HashMap::new();
        for (&x, &cnt) in &beams {
            if y[x]=='^' {
                if x>0 {
                    newbeams.entry(x-1).and_modify(|e| *e += cnt).or_insert(cnt);
                }
                if x+1<w {
                    newbeams.entry(x+1).and_modify(|e| *e += cnt).or_insert(cnt);
                }
            } else {
                newbeams.entry(x).and_modify(|e| *e += cnt).or_insert(cnt);
            }
        }
        beams = newbeams;
    }
    beams.values().sum::<i64>().to_string()
}

fn d8_parse(input: Vec<String>) -> (Vec<(i32, i32, i32)>, BinaryHeap<(Reverse<i64>, usize, usize)>) {
    let input: Vec<_> = input.into_iter().map(|line| {
        let mut parts = line.split(',');
        let x = parts.next().unwrap().parse::<i32>().unwrap();
        let y = parts.next().unwrap().parse::<i32>().unwrap();
        let z = parts.next().unwrap().parse::<i32>().unwrap();
        (x, y, z)
    }).collect();

    let mut dists = BinaryHeap::new();
    for (i, (x1, y1, z1)) in input.iter().enumerate() {
        for (j, (x2, y2, z2)) in input.iter().skip(i+1).enumerate() {
            let dist = ((x1 - x2) as i64).pow(2) + 
                ((y1 - y2) as i64).pow(2) + 
                ((z1 - z2) as i64).pow(2);
            dists.push((Reverse(dist), i,j+i+1));
        }
    }

    (input, dists)
}

pub fn d8a(input: Vec<String>) -> String {
    let (input, mut dists) = d8_parse(input);

    let max_iter = if input.len() < 100 { 10 } else { 1000 };
    let mut uf = UnionFind::new(input.len());
    let mut iter = 0;
    while let Some((_, i, j)) = dists.pop() && iter < max_iter {
        uf.union(i, j);
        iter+=1;
    }

    let mut groups = HashMap::new();
    for i in 0..input.len() {
        let root = uf.find(i);
        let size = uf.size(i);
        groups.insert(root, size);
    }

    let mut groups: Vec<_> = groups.into_iter().map(|(root, cnt)| (cnt, root)).collect();
    groups.sort_unstable();
    groups.into_iter().rev().take(3).fold(1, |acc, (cnt, _)| acc * cnt).to_string()
}

pub fn d8b(input: Vec<String>) -> String {
    let (input, mut dists) = d8_parse(input);

    let max_iter = if input.len() < 100 { 10 } else { 1000 };
    let mut uf = UnionFind::new(input.len());
    let mut iter = 0;
    while let Some((_, i, j)) = dists.pop() {
        let p = uf.union(i, j);
        if uf.size(p) == input.len() {
            return (input[i].0 * input[j].0).to_string()
        }
    }
    unreachable!()
}

fn d9_parse(input: Vec<String>) -> Vec<(i32, i32)> {
    input.into_iter().map(|line| {
        let mut line = line.split(',');
        (line.next().unwrap().parse::<i32>().unwrap(), line.next().unwrap().parse::<i32>().unwrap())
    }).collect()
}

pub fn d9a(input: Vec<String>) -> String {
    let input: Vec<_> = d9_parse(input);
    let mut ans = 0;
    for (x1,y1) in &input {
        for (x2, y2) in &input {
            let area = ((x2 - x1).abs() + 1) as i64 * ((y2 - y1).abs() + 1) as i64;
            ans = ans.max(area);
        }
    }
    ans.to_string()
}

pub fn d9b(input: Vec<String>) -> String {
    let input: Vec<_> = d9_parse(input);
    
    let mut xs: Vec<_> = input.iter().map(|(x,_)|*x).collect();
    let mut ys: Vec<_> = input.iter().map(|(_,y)|*y).collect();
    xs.sort_unstable();
    ys.sort_unstable(); 
    xs.dedup();
    ys.dedup();

    let xmap: HashMap<_,_> = xs.into_iter().enumerate().map(|(i, x)| (x, i)).collect();
    let ymap: HashMap<_,_> = ys.into_iter().enumerate().map(|(i, y)| (y, i)).collect();

    let mut grid = vec![vec![0u8; xmap.len()]; ymap.len()];

    fn drawto(grid: &mut Vec<Vec<u8>>, prev: (usize, usize), to: (usize, usize)) {
        let (dy,dx) = (to.0 as isize - prev.0 as isize, to.1 as isize - prev.1 as isize);
        assert!(dy == 0 || dx == 0);
        for step in 1..=dy.abs().max(dx.abs()) {
            let ny = (prev.0 as isize + dy.signum() * step) as usize;
            let nx = (prev.1 as isize + dx.signum() * step) as usize;
            grid[ny][nx] = 1;
        }
    }

    let mut prev=(0,0);
    for (x, y) in &input {
        let xi = xmap[x];
        let yi = ymap[y];
        if prev != (0, 0) {
            drawto(&mut grid, prev, (yi, xi));
        } else {
            grid[yi][xi] = 1;
        }
        prev = (yi, xi);
    }
    //drawto(&mut grid, prev, (xmap[&0], ymap[&0]));
    println!("{}", vvu2str(&grid));
    "".to_string()
}

fn d10_parse(input: Vec<String>) -> Vec<(i32, usize, Vec<i32>, Vec<Vec<usize>>, Vec<i32>)> {
    let re = Regex::new(r"\[([.#]+)\] ([^{]+) \{(.+)\}").unwrap();
    let btn = Regex::new(r"\(([\d,]+)\)").unwrap();
    input.into_iter().map(|line|  {
        // [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
        let caps = re.captures(&line).unwrap();
        
        let light = &caps[1];
        let length = light.len();
        let light = light.chars().rev().fold(0, |acc, c| acc*2 + if c=='.' {0} else {1});

        let buttons = &caps[2];
        let buttons = btn.captures_iter(buttons)
            .map(|bcap| bcap[1].split(',').map(|s| s.parse::<usize>().unwrap()).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let buttonsmask = buttons.iter().map(|bs| {
            bs.iter().fold(0, |acc, &btn| acc | (1 << btn))
        }).collect::<Vec<_>>();

        let volts = &caps[3];
        let volts = volts.split(',').map(|s| s.parse::<i32>().unwrap()).collect();

        println!("light: {} length: {} buttons: {:?} buttonsmask: {:?} volts: {:?}", light, length, buttons, buttonsmask, volts);
        (light, length, buttonsmask, buttons, volts)
    }).collect()
}

pub fn d10a(input: Vec<String>) -> String {
    let input = d10_parse(input);
    let mut ans = 0;
    for (light, _, buttons, _, _) in input {
        let mut q = BinaryHeap::new();
        q.push((Reverse(0), 0));
        let mut best: HashMap<i32, i32> = HashMap::new();
        best.insert(0, 0);
        while let Some((Reverse(steps), state)) = q.pop() {
            if state == light {
                ans += steps;
                break;
            }

            for &btn in &buttons {
                let newstate = state ^ btn;
                if !best.contains_key(&newstate) || best[&newstate]>steps+1 {
                    q.push((Reverse(steps+1), newstate));
                    best.insert(newstate, steps+1);
                }
            }
        }
        // println!("light: {:05b} steps: {}", light, ans);
    }
    ans.to_string()
}

pub fn d10b(input: Vec<String>) -> String {
    let input = d10_parse(input);
    let mut ans = 0;
    for (_, length, _, buttons, volts) in input {
        let mut q = BinaryHeap::new();
        q.push((Reverse(0), vec![0; length]));
        while let Some((Reverse(steps), state)) = q.pop() {
            if state == volts {
                ans += steps;
                break;
            }

            for btns in &buttons {
                let mut newstate = state.clone();
                let mut ok = true;
                for &b in btns {
                    newstate[b] += 1;
                    if newstate[b] > volts[b] {
                        ok = false;
                        break;
                    }
                }
                println!("  state: {:?} btns: {:?} -> newstate: {:?} ok: {}", state, btns, newstate, ok);
                if ok {
                    q.push((Reverse(steps+1), newstate));
                }
            }
        }
        println!("volts: {:?} steps: {}", volts, ans);
    }
    ans.to_string()
}

fn d11_parse(input: &Vec<String>) -> (HashMap<&str, usize>, HashMap<usize, Vec<usize>>) {
    let mut tos: HashMap<usize, Vec<usize>> = HashMap::new();

    let mut id: HashMap<&str, usize> = HashMap::new();

    for i in input {
        let mut ft = i.split(": ");
        let f = ft.next().unwrap();
        let t = ft.next().unwrap();

        let nextid = id.len();
        let fid = id.entry(f).or_insert(nextid).clone();

        for to in t.split_ascii_whitespace() {
            let nextid = id.len();
            let tid = id.entry(to).or_insert(nextid).clone();
            tos.entry(fid).or_insert(vec![]).push(tid);
        }
    }
    (id, tos)
}

pub fn d11a(input: Vec<String>) -> String {
    let (ids, tos) = d11_parse(&input);
    fn dfs(tos: &HashMap<usize, Vec<usize>>, cur: usize, to: usize) -> i32 {
        if cur == to { return 1 }
        let mut ans = 0;
        if let Some(tos_) = tos.get(&cur) {
            for &t in tos_ {
                ans+=dfs(tos, t, to)
            }
        }
        // println!("from: {cur} to: {to} ans: {ans}");
        ans
    }
    dfs(&tos, ids.get(&"you").unwrap().clone(), ids.get(&"out").unwrap().clone()).to_string()
}

pub fn d11b(input: Vec<String>) -> String {
    // 1650109440 too low
    let (ids, tos) = d11_parse(&input);
    fn dfs(tos: &HashMap<usize, Vec<usize>>, visited: &mut HashSet<(usize, bool, bool)>, cache: &mut HashMap<(usize, bool, bool), i64>, cur: usize, to: usize, dac: usize, fft: usize, dac_passed: bool, fft_passed: bool) -> i64 {
        if cur == to { 
            // if dac_passed && fft_passed {
            //     println!("dac {dac_passed} fft {fft_passed}");
            // }
            return if dac_passed && fft_passed {1} else {0} 
        }
        // if visited.contains(&(cur, dac_passed, fft_passed)) { 
        //     println!("visited: {cur}");
        //     return 0; 
        // }
        // visited.insert((cur, dac_passed, fft_passed));

        let mut ans = 0;
        let d_pass = dac_passed || cur == dac;
        let f_pass = fft_passed || cur == fft;
        if let Some(v) = cache.get(&(cur, dac_passed, fft_passed)) {
            return *v;
        }

        if let Some(tos_) = tos.get(&cur) {
            for &t in tos_ {
                ans += dfs(tos, visited, cache, t, to, dac, fft, d_pass, f_pass);
            }
        }
        // println!("from: {cur} to: {to} ans: {ans} dac:{d_pass} fft:{f_pass}");
        // visited.remove(&(cur, dac_passed, fft_passed));
        cache.insert((cur, d_pass, f_pass), ans);
        ans
    }
    // println!("{ids:#?}");
    dfs(&tos,   &mut HashSet::new(), &mut HashMap::new(), ids.get(&"svr").unwrap().clone(), ids.get(&"out").unwrap().clone(), 
        ids.get(&"dac").unwrap().clone(), ids.get(&"fft").unwrap().clone(), false, false).to_string()
}
