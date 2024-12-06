use std::iter::zip;

fn report(line: &str) -> Vec<i32> {
    line
        .split_whitespace()
        .map(|word|{word.parse::<i32>().unwrap()})
        .collect()
}

fn safe_report(levels: &[i32]) -> bool {
    let diffs: Vec<i32> = zip(levels.iter(), levels[1..].iter())
        .map(|(a, b)|{a - b})
        .collect();

    diffs.iter().all(|d|{d.abs() >= 1 && d.abs() <= 3})
        && zip(diffs.iter(), diffs[1..].iter()).all(|(&da, &db)|{
            (da > 0) == (db > 0)
        })
}

pub fn parse_1(text: &str) -> i32 {
    let mut n_safe = 0;

    for line in text.lines() {
        let levels = report(line);

        if levels.is_empty() {
            continue;
        }

        if safe_report(&levels) {
            n_safe += 1;
        }
    }

    n_safe
}


pub fn parse_2(text: &str) -> i32 {
    let mut n_safe = 0;

    for line in text.lines() {
        let levels = report(line);

        if levels.is_empty() {
            continue;
        }

        if safe_report(&levels) {
            n_safe += 1;
        } else {
            for i in 0..levels.len() {
                let mut l = levels.clone();
                l.remove(i);
                if safe_report(&l) {
                    n_safe += 1;
                    break;
                }
            }
        }
    }

    n_safe
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEXT_1: &str = "
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
    #[test]
    fn test_parse1() {
        assert_eq!(parse_1(&INPUT_TEXT_1), 2);
    }
    #[test]
    fn test_parse2() {
        assert_eq!(parse_2(&INPUT_TEXT_1), 4);
    }
}
