use itertools;
use itertools::Itertools;
use std::collections::BTreeSet;

fn main() {
    let input = include_str!("input1.txt");
    let answer = solve(input);
    println!("{}", answer);
}

fn solve(str: &str) -> usize {
    let grid = str.lines().collect::<Vec<_>>();
    let mut galaxy_coords = Vec::new();
    let mut double_rows = BTreeSet::new();
    let mut double_cols = BTreeSet::new();
    for (i, row) in grid.iter().enumerate() {
        let mut galaxy_in_row = false;
        for (j, byte) in row.bytes().enumerate() {
            if byte == b'#' {
                galaxy_in_row = true;
                galaxy_coords.push((i, j));
            }
        }
        if !galaxy_in_row {
            double_rows.insert(i);
        }
    }
    for j in 0..grid[0].len() {
        if !grid.iter().any(|row| row.as_bytes()[j] == b'#') {
            double_cols.insert(j);
        }
    }
    galaxy_coords
        .into_iter()
        .tuple_combinations()
        .map(|((i1, j1), (i2, j2))| {
            let (min_i, max_i) = if i1 < i2 { (i1, i2) } else { (i2, i1) };
            let (min_j, max_j) = if j1 < j2 { (j1, j2) } else { (j2, j1) };
            let double_rows_count = double_rows.range(min_i..max_i).count();
            let double_columns_count = double_cols.range(min_j..max_j).count();
            (max_j - min_j) + (max_i - min_i) + double_rows_count + double_columns_count
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        let res = solve(&input);

        assert_eq!(res, 374);
    }
}
