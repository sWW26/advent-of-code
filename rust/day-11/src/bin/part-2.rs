use itertools;
use itertools::Itertools;
use std::collections::BTreeSet;

fn main() {
    let input = include_str!("input2.txt");
    let answer = solve(input, 1_000_000);
    println!("{}", answer);
}

fn solve(str: &str, empty_size: usize) -> usize {
    let grid = str.lines().collect::<Vec<_>>();
    let mut galaxy_coords = Vec::new();
    let mut empty_rows = BTreeSet::new();
    let mut empty_cols = BTreeSet::new();
    for (i, row) in grid.iter().enumerate() {
        let mut galaxy_in_row = false;
        for (j, byte) in row.bytes().enumerate() {
            if byte == b'#' {
                galaxy_in_row = true;
                galaxy_coords.push((i, j));
            }
        }
        if !galaxy_in_row {
            empty_rows.insert(i);
        }
    }
    for j in 0..grid[0].len() {
        if !grid.iter().any(|row| row.as_bytes()[j] == b'#') {
            empty_cols.insert(j);
        }
    }
    // Subtract one to avoid double counting the row that physically exists in the grid
    let empty_multiplier = empty_size - 1;
    galaxy_coords
        .into_iter()
        .tuple_combinations()
        .map(|((i1, j1), (i2, j2))| {
            let (min_i, max_i) = if i1 < i2 { (i1, i2) } else { (i2, i1) };
            let (min_j, max_j) = if j1 < j2 { (j1, j2) } else { (j2, j1) };
            let empty_rows_count = empty_rows.range(min_i..max_i).count();
            let empty_columns_count = empty_cols.range(min_j..max_j).count();
            (max_j - min_j)
                + (max_i - min_i)
                + (empty_rows_count * empty_multiplier)
                + (empty_columns_count * empty_multiplier)
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

        let res = solve(&input, 10);

        assert_eq!(res, 1030);
    }

    #[test]
    fn example2() {
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

        let res = solve(&input, 100);

        assert_eq!(res, 8410);
    }
}
