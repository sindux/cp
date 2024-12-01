pub fn d1(input: Vec<String>) -> String {
    let input: Vec<_> = input.into_iter().map(|l|{
        let l:  Vec<i32> = l.split_ascii_whitespace().map(|n|n.parse().unwrap()).collect();
        (l[0], l[1])
    }).collect();
    let (mut l1, mut l2): (Vec<i32>, Vec<i32>) = input.into_iter().unzip();
    l1.sort_unstable();
    l2.sort_unstable();
    let res: i32 = l1.into_iter().zip(l2).map(|(a,b)|(a-b).abs()).sum();
    res.to_string()
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
