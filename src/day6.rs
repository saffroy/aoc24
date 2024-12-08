#[derive(Copy,Clone,PartialEq)]
enum Direction {
    N, S, E, W
}

#[derive(Copy,Clone,PartialEq,Debug)]
enum Outcome {
    Out, Loop
}

fn parse_grid(text: &str) -> (Vec<Vec<u8>>, usize, usize) {
    let grid: Vec<Vec<u8>> = text.lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.as_bytes().to_vec())
        .collect();

    let (mut x, mut y) = (0,0);
    'outer: for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().copied().enumerate() {
            if c == b'^' {
                (x, y) = (i, j);
                break 'outer;
            }
        }
    }

    (grid, x, y)
}

fn trace_patrol(grid: &Vec<Vec<u8>>, x0: usize, y0: usize, dir0: Direction)
                -> (Vec<Vec<bool>>, Outcome) {
    let mut dir = dir0;
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut visited_dirs =
        vec![vec![Direction::N; grid[0].len()]; grid.len()];

    let (mut x, mut y) = (x0, y0);
    visited[x][y] = true;
    visited_dirs[x][y] = dir0;

    loop {
        let (xnext, ynext) = match dir {
            Direction::N => (x.overflowing_sub(1).0, y),
            Direction::S => (x+1, y),
            Direction::E => (x, y+1),
            Direction::W => (x, y.overflowing_sub(1).0),
        };

        match grid.get(xnext).and_then(|row| row.get(ynext)) {
            None => return (visited, Outcome::Out),
            Some(val) =>
                match val {
                    b'#' => dir = match dir {
                        Direction::N => Direction::E,
                        Direction::E => Direction::S,
                        Direction::S => Direction::W,
                        Direction::W => Direction::N,
                    },
                    _ => {
                        if visited[xnext][ynext]
                            && visited_dirs[xnext][ynext] == dir {
                                return (visited, Outcome::Loop)
                            }
                        (x, y) = (xnext, ynext);
                        visited[x][y] = true;
                        visited_dirs[x][y] = dir;
                    },
                },
        };
    }
}

pub fn parse_1(text: &str) -> i64 {
    let (grid, x0, y0) = parse_grid(text);
    let (visited, out) = trace_patrol(&grid, x0, y0, Direction::N);
    assert_eq!(out, Outcome::Out);

    visited.iter().map(|row| {
        row.iter().copied().filter(|&p| p).count() as i64
    }).sum()
}

pub fn parse_2(text: &str) -> i64 {
    // parse grid
    // trace visited => candidates for obstacle
    // (remove init)
    // for each candidate:
    // - set obstacle
    // - run until out or loop,
    //   loops = same pos + same direction
    // - count those that loop
    let (mut grid, x0, y0) = parse_grid(text);
    let (visited, _) = trace_patrol(&grid, x0, y0, Direction::N);

    visited.iter().enumerate().map(|(i, row)| {
        row.iter().enumerate().map(|(j, _)| j)
            .filter(|&j| visited[i][j] && (i, j) != (x0, y0))
            .map(|j| {
                grid[i][j] = b'#';
                let (_, out) = trace_patrol(&grid, x0, y0, Direction::N);
                grid[i][j] = b'.';
                out
            })
            .filter(|&out| out == Outcome::Loop)
            .count()
    }).sum::<usize>() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEXT_1: &str = "
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";
    #[test]
    fn test_parse1() {
        assert_eq!(parse_1(&INPUT_TEXT_1), 41);
    }
    #[test]
    fn test_parse2() {
        assert_eq!(parse_2(&INPUT_TEXT_1), 6);
    }
}
