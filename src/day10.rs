use std::collections::HashSet;

fn neighbours(grid: &Vec<Vec<i32>>, i: usize, j: usize)
              -> Vec<(usize, usize)> {
    let mut v = vec![];

    if i > 0 {
        v.push((i-1, j));
    }
    if i < grid.len()-1 {
        v.push((i+1, j));
    }
    if j > 0 {
        v.push((i, j-1));
    }
    if j < grid[0].len()-1 {
        v.push((i, j+1));
    }

    v
}

fn parse_common(text: &str) -> Vec<Vec<i32>> {
    text.lines()
        .filter(|&line| !line.is_empty())
        .map(|line| {
            line.as_bytes().iter()
                .map(|c| (c - b'0') as i32).collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn parse_1(text: &str) -> i64 {
    let grid = parse_common(text);

    let mut nines =
        vec![vec![HashSet::<(usize,usize)>::new(); grid[0].len()]; grid.len()];

    let mut score = 0;

    for height in (0..9+1).rev() {
        for (i, row) in grid.iter().enumerate() {
            for (j, &c) in row.iter().enumerate() {
                if c == height {
                    let mut s = HashSet::new();
                    if c == 9 {
                        s.insert((i, j));
                    } else {
                        neighbours(&grid, i, j).iter()
                            .filter(|(x, y)| grid[*x][*y] == c+1)
                            .for_each(|(x, y)|
                                      s.extend(&nines[*x][*y]));
                    }
                    if c == 0 {
                        score += s.len();
                    }
                    nines[i][j] = s;
                }
            }
        }
    }

    score as i64
}

pub fn parse_2(text: &str) -> i64 {
    let grid = parse_common(text);

    let mut trails = vec![vec![0; grid[0].len()]; grid.len()];

    let mut score = 0;

    for height in (0..9+1).rev() {
        for (i, row) in grid.iter().enumerate() {
            for (j, &c) in row.iter().enumerate() {
                if c == height {
                    if c == 9 {
                        trails[i][j] = 1;
                    } else {
                        let s = neighbours(&grid, i, j).iter()
                            .filter(|(x, y)| grid[*x][*y] == c+1)
                            .map(|(x, y)| trails[*x][*y])
                            .sum();
                        trails[i][j] = s;
                        if c == 0 {
                            score += s;
                        }
                    }
                }
            }
        }
    }

    score as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEXT_1: &str = "
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";
    #[test]
    fn test1_parse1() {
        assert_eq!(parse_1(&INPUT_TEXT_1), 36);
    }
    #[test]
    fn test1_parse2() {
        assert_eq!(parse_2(&INPUT_TEXT_1), 81);
    }
}
