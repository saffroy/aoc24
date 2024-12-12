use std::collections::{HashMap, HashSet};

#[derive(Clone,Copy)]
struct Tile {
    plant: u8,
    connected: u8,
    component: i32,
}

fn neighbours<T>(grid: &[Vec<T>], i: usize, j: usize) -> Vec<(usize, usize)> {
    let v = vec![
        (i.overflowing_sub(1).0, j),
        (i, j.overflowing_sub(1).0),
        (i+1, j),
        (i, j+1),
    ];

    v.into_iter()
        .filter(|(x, y)| grid.get(*x).and_then(|row| row.get(*y)).is_some())
        .collect::<Vec<(usize, usize)>>()
}

fn component_add(components: &mut HashMap<i32, HashSet<(usize,usize)>>,
                 component: i32,
                 grid: &mut [Vec<Tile>],
                 i: usize, j: usize) {
    let s = components.entry(component).or_default();
    s.insert((i, j));
    grid[i][j].component = component;
}

fn component_merge(components: &mut HashMap<i32, HashSet<(usize,usize)>>,
                   comp_a: i32, comp_b: i32,
                   grid: &mut [Vec<Tile>]) {
    let sb = components.remove(&comp_b).unwrap();
    let sa = components.get_mut(&comp_a).unwrap();
    for &(x, y) in sb.iter() {
        grid[x][y].component = comp_a;
    }
    sa.extend(sb.iter());
}

fn parse_common(text: &str) -> Vec<Vec<Tile>> {
    text.lines()
        .filter(|line| !line.is_empty())
        .map(|line| line
             .as_bytes()
             .iter()
             .map(|&c| Tile { plant: c, connected: 0, component: 0 })
             .collect())
        .collect()
}

type Components = HashMap<i32, HashSet<(usize, usize)>>;

fn compute_components(grid: &mut Vec<Vec<Tile>>) -> Components {
    // component id is number
    // for each tile:
    // - count connected neighbours == those with same plant
    //   fence count == 4 - connected
    // - if tile to the left connected: assign cur tile to its component
    // - if tile above connected:
    //   - if cur tile already connected: merge components
    //     else: assign cur tile to above component
    // - if cur tile not connected: assign to new component

    let mut comp_id = 1;
    let mut components = HashMap::new();

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let mut tile = grid[i][j];
            for &(x, y) in neighbours(grid, i, j).iter() {
                let n = &mut grid[x][y];

                if tile.plant == n.plant {
                    tile.connected += 1;

                    if x < i || y < j {
                        if tile.component == 0 {
                            assert!(n.component > 0);
                            tile.component = n.component;
                            component_add(&mut components, n.component,
                                          grid, i, j);
                        } else if tile.component != n.component {
                            component_merge(&mut components,
                                            n.component,
                                            tile.component,
                                            grid);
                            tile.component = grid[i][j].component;
                        }
                    } else {
                        assert!(n.component == 0);
                    }
                }
            }
            if tile.component == 0 {
                component_add(&mut components, comp_id,
                              grid, i, j);
                comp_id += 1;
            }
            grid[i][j].connected = tile.connected;
        }
    }

    components
}

pub fn parse_1(text: &str) -> i64 {
    let mut grid = parse_common(text);
    let components = compute_components(&mut grid);

    components.values()
        .map(|s| {
            let perimeter = s.iter()
                .map(|&(x, y)| 4 - grid[x][y].connected as usize)
                .sum::<usize>();
            s.len() * perimeter
        })
        .sum::<usize>() as i64
}

pub fn parse_2(_text: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEXT_1: &str = "
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";
    #[test]
    fn test1_parse1() {
        assert_eq!(parse_1(&INPUT_TEXT_1), 1930);
    }
    #[test]
    fn test1_parse2() {
        assert_eq!(parse_2(&INPUT_TEXT_1), 1206);
    }
}
