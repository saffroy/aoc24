use regex::Regex;

fn parse_line(line: &str) -> Option<(i64, Vec<i64>)> {
    let re_head = Regex::new(r"(\d+): *(.*)").unwrap();
    let re_tail = Regex::new(r"(\d+) *").unwrap();

    match re_head.captures(line) {
        None => None,
        Some(caps) => {
            let head: i64 = caps[1].parse().unwrap();
            let args: Vec<i64> = re_tail.captures_iter(&caps[2])
                .map(|caps| caps[1].parse::<i64>().unwrap())
                .collect();
            Some((head, args))
        },
    }
}

fn solvable_p(goal: i64, curr: i64, args: &[i64]) -> bool {
    if args.is_empty() {
        curr == goal
    } else if curr > goal {
        false
    } else {
        solvable_p(goal, curr * args[0], &args[1..])
            || solvable_p(goal, curr + args[0], &args[1..])
    }
}

pub fn parse_1(text: &str) -> i64 {
    text.lines()
        .filter_map(parse_line)
        .filter(|(head, args)| solvable_p(*head, args[0], &args[1..]))
        .map(|(head, _)| head)
        .sum()
}

fn concat(a: i64, b: i64) -> i64 {
    let s = format!("{}{}", a, b);
    s.parse::<i64>().unwrap()
}

fn solvable2_p(goal: i64, curr: i64, args: &[i64]) -> bool {
    if args.is_empty() {
        curr == goal
    } else if curr > goal {
        false
    } else {
        solvable2_p(goal, curr * args[0], &args[1..])
            || solvable2_p(goal, curr + args[0], &args[1..])
            || solvable2_p(goal, concat(curr, args[0]), &args[1..])
    }
}

pub fn parse_2(text: &str) -> i64 {
    text.lines()
        .filter_map(parse_line)
        .filter(|(head, args)| solvable2_p(*head, args[0], &args[1..]))
        .map(|(head, _)| head)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEXT_1: &str = "
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";
    #[test]
    fn test_parse1() {
        assert_eq!(parse_1(&INPUT_TEXT_1), 3749);
    }
    #[test]
    fn test_parse2() {
        assert_eq!(parse_2(&INPUT_TEXT_1), 11387);
    }
}
