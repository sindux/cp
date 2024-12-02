
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

pub fn d2a(input: Vec<String>) -> String {
    fn is_diff_ok(first_diff: i32, cur_diff: i32) -> bool {
        let ca = cur_diff.abs();
        ca>=1 && ca<=3 && first_diff.signum() == cur_diff.signum()
    }
    fn is_safe(level: &str) -> bool {
        let levels: Vec<i32> = level.split_ascii_whitespace().map(|n|n.parse().unwrap()).collect();
        let first_diff = levels[1]-levels[0];
        for i in 1..levels.len() {
            let cur_diff = levels[i] - levels[i-1];
            if !is_diff_ok(first_diff, cur_diff) {
                return false
            }
        }
        true
    }
    input.into_iter().filter(|l| is_safe(l)).count().to_string()
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
