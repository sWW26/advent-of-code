fn main() {
    let input = include_str!("input1.txt");
    let answer = solve(input);
    println!("{}", answer);
}

const START: u8 = b'S';

fn find_starting_location(grid: &Vec<Vec<u8>>) -> Option<(usize, usize)> {
    for (i, x) in grid.iter().enumerate() {
        for (j, x) in x.into_iter().enumerate() {
            if *x == START {
                return Some((i, j));
            }
        }
    }

    None
}

fn construct_grid(str: &str) -> Vec<Vec<u8>> {
    str.lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<_>>()
}

enum ConnectedToTiles {
    StartTile,
    Ground,
    Pipe([(i8, i8); 2]),
}

fn get_linked_tiles(tile: u8) -> ConnectedToTiles {
    match tile {
        b'|' => ConnectedToTiles::Pipe([(-1, 0), (1, 0)]),
        b'-' => ConnectedToTiles::Pipe([(0, -1), (0, 1)]),
        b'L' => ConnectedToTiles::Pipe([(-1, 0), (0, 1)]),
        b'J' => ConnectedToTiles::Pipe([(-1, 0), (0, -1)]),
        b'7' => ConnectedToTiles::Pipe([(1, 0), (0, -1)]),
        b'F' => ConnectedToTiles::Pipe([(1, 0), (0, 1)]),
        b'.' => ConnectedToTiles::Ground,
        START => ConnectedToTiles::StartTile,
        _ => panic!("unexpected tile {}", tile as char),
    }
}

fn invert_direction((a, b): (i8, i8)) -> (i8, i8) {
    (a * -1, b * -1)
}

fn apply_single_direction(x: usize, dir: i8, max: usize) -> Option<usize> {
    if dir.is_negative() {
        x.checked_sub(dir.abs() as usize)
    } else {
        let new = x + dir as usize;
        if new > max {
            None
        } else {
            Some(new)
        }
    }
}

fn apply_direction(
    (a, b): (usize, usize),
    (direction_a, direction_b): (i8, i8),
    max_a: usize,
    max_b: usize,
) -> Option<(usize, usize)> {
    apply_single_direction(a, direction_a, max_a)
        .and_then(|new_a| apply_single_direction(b, direction_b, max_b).map(|new_b| (new_a, new_b)))
}

fn try_walk(
    grid: &Vec<Vec<u8>>,
    mut current_tile: (usize, usize),
    mut came_from: (i8, i8),
) -> Option<usize> {
    let mut steps = 0;
    loop {
        steps += 1;
        match get_linked_tiles(grid[current_tile.0][current_tile.1]) {
            ConnectedToTiles::Pipe(x) => {
                if !x.contains(&came_from) {
                    break None;
                } else {
                    let out_direction = if x[0] == came_from { x[1] } else { x[0] };
                    if let Some(new_tile) =
                        apply_direction(current_tile, out_direction, grid.len(), grid[0].len())
                    {
                        current_tile = new_tile;
                        came_from = invert_direction(out_direction);
                    } else {
                        break None;
                    }
                }
            }
            ConnectedToTiles::Ground => break None,
            ConnectedToTiles::StartTile => break Some(steps),
        }
    }
}

fn solve(str: &str) -> usize {
    let grid = construct_grid(str);
    let starting_tile = find_starting_location(&grid).expect("must have a starting location");
    let total_steps = [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .into_iter()
        .find_map(|direction| {
            apply_direction(starting_tile, direction, grid.len(), grid[0].len())
                .and_then(|tile| try_walk(&grid, tile, invert_direction(direction)))
        })
        .expect("must be at least one loop");
    (total_steps + 1) / 2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_find_starting_location() {
        let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

        let grid = construct_grid(&input);
        let res = find_starting_location(&grid);

        assert_eq!(res, Some((1, 1)));
    }

    #[test]
    fn example1() {
        let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

        let res = solve(&input);

        assert_eq!(res, 4);
    }

    #[test]
    fn example2() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

        let res = solve(&input);

        assert_eq!(res, 8);
    }
}
