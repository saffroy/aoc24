use memoize::memoize;

fn blink(n: i64) -> Vec<i64> {
    let mut v = vec![];

    if n == 0 {
        v.push(1);
    } else {
        let s = format!("{}", n);
        if s.len() % 2 == 0 {
            let d1: i64 = s[..s.len()/2].parse().unwrap();
            let d2: i64 = s[s.len()/2..].parse().unwrap();
            v.push(d1);
            v.push(d2);
        } else {
            v.push(n * 2024);
        }
    }

    v
}

#[memoize]
fn count_stones(n: i64, blinks: i64) -> i64 {
    if blinks == 0 {
        1
    } else {
        blink(n)
            .iter()
            .map(|&x| count_stones(x, blinks-1))
            .sum()
    }
}

fn parse_common(text: &str) -> Vec<i64> {
    text.lines()
        .find(|line| !line.is_empty())
        .unwrap()
        .split(' ')
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

pub fn parse_1(text: &str) -> i64 {
    let nums = parse_common(text);
    nums.iter().map(|&x| count_stones(x, 25)).sum()
}

pub fn parse_2(text: &str) -> i64 {
    let nums = parse_common(text);
    nums.iter().map(|&x| count_stones(x, 75)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEXT_1: &str = "
125 17
";
    #[test]
    fn test1_parse1() {
        assert_eq!(parse_1(&INPUT_TEXT_1), 55312);
    }
    #[test]
    fn test1_parse2() {
        assert_eq!(parse_2(&INPUT_TEXT_1), 65601038650482);
    }
}
