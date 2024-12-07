use regex::Regex;
use std::{collections::HashMap, cmp::Ordering};

type RuleSet = HashMap<(i32,i32), i32>;

fn parse_common(text: &str) -> (RuleSet, Vec<Vec<i32>>) {
    let re_rule = Regex::new(r"(\d+)\|(\d+)").unwrap();
    let re_page = Regex::new(r"(\d+),?").unwrap();
    let mut rules = HashMap::new();
    let mut updates = Vec::new();

    for line in text.lines() {
        if line.is_empty() {
            continue;
        } else if let Some(caps) = re_rule.captures(line) {
            let a: i32 = caps[1].parse().unwrap();
            let b: i32 = caps[2].parse().unwrap();
            rules.insert((a, b), 1);
        } else {
            let vec: Vec<i32> = re_page.captures_iter(line)
                .map(|caps| caps[1].parse::<i32>().unwrap())
                .collect();
            updates.push(vec);
        }
    }

    (rules, updates)
}

fn update_correct_p(rules: &RuleSet, update: &[i32]) -> bool {
    for (i1, p1) in update.iter().enumerate() {
        for p2 in update.iter().skip(i1+1) {
            if rules.contains_key(&(*p2, *p1)) {
                return false;
            }
        }
    }
    true
}

pub fn parse_1(text: &str) -> i32 {
    let (rules, updates) = parse_common(text);
    let mut sum = 0;

    for vec in updates {
        if update_correct_p(&rules, &vec) {
            sum += vec[vec.len()/2];
        }
    }

    sum
}

pub fn parse_2(text: &str) -> i32 {
    let (rules, updates) = parse_common(text);
    let mut sum = 0;

    for mut vec in updates {
        if !update_correct_p(&rules, &vec) {
            vec.sort_by(|a,b| {
                if a == b {
                    Ordering::Equal
                } else if rules.contains_key(&(*a, *b)) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
            sum += vec[vec.len()/2];
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEXT_1: &str = "
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";
    #[test]
    fn test_parse1() {
        assert_eq!(parse_1(&INPUT_TEXT_1), 143);
    }
    #[test]
    fn test_parse2() {
        assert_eq!(parse_2(&INPUT_TEXT_1), 123);
    }
}
