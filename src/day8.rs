use std::collections::HashMap;

type AntennaMap = HashMap<u8, Vec<(i32, i32)>>;

fn parse_common(text: &str) -> (Vec<Vec<u8>>, AntennaMap) {
    let grid: Vec<_> = text.lines()
        .filter(|&line| !line.is_empty())
        .map(|line| line.as_bytes().to_vec())
        .collect();

    let mut antennas = HashMap::new();

    for (i, row) in grid.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c != b'.' {
                let v = antennas.entry(c).or_insert(vec![]);
                v.push((i as i32, j as i32));
            }
        }
    }

    (grid, antennas)
}

fn set_loc(locs: &mut [Vec<bool>], x: i32, y: i32, out: &mut bool) {
    if let Some(c) = locs.get_mut(x as usize)
        .and_then(|row| row.get_mut(y as usize)) {
            *c = true;
            *out = false;
        }
    else {
        *out = true;
    }
}

pub fn parse_1(text: &str) -> i64 {
    let (grid, antennas) = parse_common(text);
    let mut locs = vec![vec![false; grid[0].len()]; grid.len()];
    let mut dummy: bool = false;

    for (_, v) in antennas.iter() {
        for (i, (xa, ya)) in v.iter().enumerate() {
            for (xb, yb) in v[i+1..].iter() {
                let (dx, dy) = (xb - xa, yb - ya);
                let (px, py) = (xa - dx, ya - dy);
                let (qx, qy) = (xb + dx, yb + dy);

                set_loc(&mut locs, px, py, &mut dummy);
                set_loc(&mut locs, qx, qy, &mut dummy);
            }
        }
    }

    locs.iter()
        .map(|row| row.iter().filter(|&p| *p).count())
        .sum::<usize>() as i64
}

pub fn parse_2(text: &str) -> i64 {
    let (grid, antennas) = parse_common(text);
    let mut locs = vec![vec![false; grid[0].len()]; grid.len()];

    for (_, v) in antennas.iter() {
        for (i, &(xa, ya)) in v.iter().enumerate() {
            for &(xb, yb) in v[i+1..].iter() {
                let (dx, dy) = (xb - xa, yb - ya);

                let (mut x, mut y) = (xa, ya);
                loop {
                    let mut out: bool = false;
                    set_loc(&mut locs, x, y, &mut out);
                    if out {
                        break;
                    }
                    x -= dx;
                    y -= dy;
                }

                let (mut x, mut y) = (xb, yb);
                loop {
                    let mut out: bool = false;
                    set_loc(&mut locs, x, y, &mut out);
                    if out {
                        break;
                    }
                    x += dx;
                    y += dy;
                }
            }
        }
    }

    locs.iter()
        .map(|row| row.iter().filter(|&p| *p).count())
        .sum::<usize>() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEXT_1: &str = "
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";
    #[test]
    fn test_parse1() {
        assert_eq!(parse_1(&INPUT_TEXT_1), 14);
    }
    #[test]
    fn test_parse2() {
        assert_eq!(parse_2(&INPUT_TEXT_1), 34);
    }
}
