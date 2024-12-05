use regex::Regex;
use std::collections::HashMap;

pub fn parse_1(text: &str) -> i32 {
    let re = Regex::new(r"(\d+) +(\d+)").unwrap();
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    for line in text.lines() {
        if let Some(caps) = re.captures(line) {
            let left_int: i32 = caps[1].parse().unwrap();
            let right_int: i32 = caps[2].parse().unwrap();
            left.push(left_int);
            right.push(right_int);
        }
    }

    left.sort();
    right.sort();

    let mut dist = 0;
    for (i, &l) in left.iter().enumerate() {
        let r = right[i];
        dist += (l - r).abs();
    }

    dist
}

pub fn parse_2(text: &str) -> i32 {
    let re = Regex::new(r"(\d+) +(\d+)").unwrap();
    let mut left: Vec<i32> = vec![];
    let mut right = HashMap::new();

    for line in text.lines() {
        if let Some(caps) = re.captures(line) {
            let left_int: i32 = caps[1].parse().unwrap();
            let right_int: i32 = caps[2].parse().unwrap();

            left.push(left_int);

            let rval = right.entry(right_int).or_insert(0);
            *rval += 1;
        }
    }

    let mut dist = 0;
    for &l in left.iter() {
        let rval = right.entry(l).or_insert(0);
        dist += l * (*rval);
    }

    dist
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEXT_1: &str = "
3   4
4   3
2   5
1   3
3   9
3   3
";
    #[test]
    fn test1_parse1() {
        assert_eq!(parse_1(&INPUT_TEXT_1), 11);
    }
    #[test]
    fn test1_parse2() {
        assert_eq!(parse_2(&INPUT_TEXT_1), 31);
    }
}
