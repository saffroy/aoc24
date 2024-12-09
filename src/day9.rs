pub fn parse_1(text: &str) -> i64 {
    let dense_map: Vec<usize> = text.lines()
        .find(|&line| !line.is_empty())
        .unwrap()
        .as_bytes()
        .iter()
        .map(|c| (c - b'0') as usize)
        .collect();

    let mut sparse_map: Vec<usize> = vec![];
    let mut next_fid = 0;
    let mut n_free = 0;

    dense_map.iter().enumerate().for_each(|(i, n)| {
        let fid = if i%2 == 0 {
            next_fid += 1;
            next_fid - 1
        } else {
            n_free += n;
            usize::MAX
        };
        sparse_map.resize(sparse_map.len() + n, fid);
    });

    let mut idx_free = 0;
    let mut idx_tail = sparse_map.len() - 1;

    while n_free > 0 {
        if sparse_map[idx_tail] == usize::MAX {
            idx_tail -= 1;
            n_free -= 1;
            continue;
        }
        while sparse_map[idx_free] != usize::MAX {
            idx_free += 1;
        }
        sparse_map[idx_free] = sparse_map[idx_tail];
        sparse_map[idx_tail] = usize::MAX;
    }

    sparse_map[0..idx_tail+1].iter()
        .enumerate()
        .map(|(i, fid)| i*fid)
        .sum::<usize>() as i64
}

pub fn parse_2(_text: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEXT_1: &str = "
2333133121414131402
";
    #[test]
    fn test_parse1() {
        assert_eq!(parse_1(&INPUT_TEXT_1), 1928);
    }
    #[test]
    fn test_parse2() {
        assert_eq!(parse_2(&INPUT_TEXT_1), 34);
    }
}
