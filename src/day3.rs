use regex::Regex;

pub fn parse_1(text: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut s = 0;

    for caps in re.captures_iter(text) {
        let left_int: i32 = caps[1].parse().unwrap();
        let right_int: i32 = caps[2].parse().unwrap();
        s += left_int * right_int;
    }

    s
}

pub fn parse_2(text: &str) -> i32 {
    let re = Regex::new(r"(do\(\))|(don't\(\))|(mul\((\d+),(\d+)\))").unwrap();
    let mut s = 0;
    let mut enabled = true;

    for caps in re.captures_iter(text) {
        if caps.get(1).is_some() {
            enabled = true;
        } else if caps.get(2).is_some() {
            enabled = false;
        } else if enabled {
            let left_int: i32 = caps[4].parse().unwrap();
            let right_int: i32 = caps[5].parse().unwrap();
            s += left_int * right_int;
        }
    }

    s
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEXT_1: &str = "
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";
    const INPUT_TEXT_2: &str = "
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
";
    #[test]
    fn test_parse1() {
        assert_eq!(parse_1(&INPUT_TEXT_1), 161);
    }
    #[test]
    fn test_parse2() {
        assert_eq!(parse_2(&INPUT_TEXT_2), 48);
    }
}
