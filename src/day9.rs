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

pub fn parse_2(text: &str) -> i64 {
    // parse
    // create fid table -> offset + size
    // create hole list -> offset + size
    // scan fid table downwards:
    // scan holes at lower offset
    // when hole found: move file, shrink/move hole
    // render sparse map, or compute chksum smarter (with ranges)

    let mut fid_table: Vec<(usize, usize)> = vec![];
    let mut hole_table: Vec<(usize, usize)> = vec![];
    let mut offset = 0;

    text.lines()
        .find(|&line| !line.is_empty())
        .unwrap()
        .as_bytes()
        .iter()
        .map(|c| (c - b'0') as usize)
        .enumerate()
        .for_each(|(i, n)| {
            if i%2 == 0 {
                fid_table.push((offset, n));
            } else {
                hole_table.push((offset, n));
            }
            offset += n;
        });

    for fid in (0..fid_table.len()).rev() {
        let (file_offset, file_length) = fid_table[fid];
        if let Some(hid) = hole_table.iter()
            .position(|&(hole_offset, hole_length)|
                  hole_offset < file_offset && hole_length >= file_length) {
                let (hole_offset, hole_length) = hole_table[hid];
                fid_table[fid] = (hole_offset, file_length);
                hole_table[hid] = (hole_offset + file_length,
                                   hole_length - file_length);

                if let Some(hid2) = hole_table.iter()
                    .position(|&(h2_offset, h2_length)|
                              h2_offset + h2_length == file_offset) {
                        let (h2_offset, mut h2_length) = fid_table[hid2];
                        h2_length += file_offset;
                        if fid_table[hid2+1].0 == h2_offset + h2_length {
                            h2_length += fid_table[hid2+1].1;
                            hole_table.remove(hid2+1);
                        }
                        hole_table[hid2] = (h2_offset, h2_length);
                    }
            }
    }

    fid_table.iter()
        .enumerate()
        .map(|(fid, (file_offset, file_length))|
             fid * ((*file_offset..file_offset+file_length).sum::<usize>()))
        .sum::<usize>() as i64
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
        assert_eq!(parse_2(&INPUT_TEXT_1), 2858);
    }
}
