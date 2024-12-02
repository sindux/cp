
use std::collections::HashMap;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = d1(vec!["a".to_string()]);
        assert_eq!(result, "4".to_string());
    }
}
