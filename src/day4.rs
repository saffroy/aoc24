use std::cmp::min;

fn count_matches(needle: &str, haystack: &str) -> i32 {
    haystack.matches(needle).count().try_into().unwrap()
}

fn scan(needle: &str, lines: &[String]) -> i32 {
    lines.iter().map(|l| count_matches(needle, l)).sum()
}

fn xscan(lines: &[String]) -> i32 {
    scan("XMAS", lines) + scan("SAMX", lines)
}

fn transpose(lines: &Vec<String>) -> Vec<String> {
    let columns = lines[0].len();
    let mut vec = Vec::with_capacity(columns);
    for _ in 0..columns {
        vec.push(String::with_capacity(lines.len()));
    }
    for line in lines.iter() {
        for (j, c) in line.chars().enumerate() {
            vec[j].push(c);
        }
    }
    vec
}

fn mirror(lines: &[String]) -> Vec<String> {
    lines.iter().map(|s| {
        let mut v: Vec<_> = s.clone().into_bytes();
        v.reverse();
        String::from_utf8(v).unwrap()
    }).collect()
}

fn diag(lines_: &[String]) -> Vec<String> {
    let lines: Vec<_> = lines_.iter().map(|s| s.as_bytes()).collect();
    let n_lines = lines.len();
    let n_cols = lines[0].len();
    let diags = n_lines + n_cols;
    let mut vec: Vec<Vec<u8>> = Vec::with_capacity(diags);
    for d in 0..diags {
        vec.push(Vec::new());
        let i0 = min(d, n_lines);
        let j0 = d - i0;
        for n in 0..i0+1 {
            let (i, j) = (i0-n, j0+n);
            if i >= lines.len() || j >= lines[i].len() {
                continue;
            }
            vec[d].push(lines[i][j]);
        }
    }
    vec.iter().map(|s| String::from_utf8(s.clone()).unwrap()).collect()
}

pub fn parse_1(text: &str) -> i64 {
    let lines: Vec<String> = text
        .lines()
        .filter(|l| !l.is_empty())
        .map(|s| s.to_string())
        .collect();
    (xscan(&lines)
     + xscan(&transpose(&lines))
     + xscan(&diag(&lines))
     + xscan(&diag(&mirror(&lines)))) as i64
}

fn checkmas(c: u8, ms: &mut i32, ss: &mut i32) {
    if c == b'M' {
        *ms += 1;
    }
    if c == b'S' {
        *ss += 1;
    }
}

pub fn parse_2(text: &str) -> i64 {
    let lines: Vec<_> = text
        .lines()
        .filter(|l| !l.is_empty())
        .map(|s| s.as_bytes())
        .collect();
    let n_lines = lines.len();
    let n_cols = lines[0].len();
    let mut count = 0;

    for i in 1..n_lines-1 {
        for j in 1..n_cols-1 {
            if lines[i][j] == b'A' {
                let mut ms: i32 = 0;
                let mut ss: i32 = 0;
                checkmas(lines[i-1][j-1], &mut ms, &mut ss);
                checkmas(lines[i+1][j-1], &mut ms, &mut ss);
                checkmas(lines[i-1][j+1], &mut ms, &mut ss);
                checkmas(lines[i+1][j+1], &mut ms, &mut ss);
                if ms == 2 && ss == 2
                    && lines[i-1][j-1] != lines[i+1][j+1] {
                    count += 1;
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEXT_1: &str = "
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";
    #[test]
    fn test_parse1() {
        assert_eq!(parse_1(&INPUT_TEXT_1), 18);
    }
    #[test]
    fn test_parse2() {
        assert_eq!(parse_2(&INPUT_TEXT_1), 9);
    }
}
