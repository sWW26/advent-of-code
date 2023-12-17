use std::collections::HashSet;

fn main() {
    let input = include_str!("input2.txt");
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

enum TurnType {
    Clockwise,
    AntiClockwise,
    Straight,
}

fn get_turn_type(start: (i8, i8), end: (i8, i8)) -> TurnType {
    let sum = (start.0 + end.0, start.1 + end.1);
    let entry_adjustment = if start.1 == 0 { 1 } else { -1 };
    match sum.0 * sum.1 * entry_adjustment {
        0 => TurnType::Straight,
        1 => TurnType::Clockwise,
        -1 => TurnType::AntiClockwise,
        _ => panic!("turn type calculation should always be 0, 1 or -1"),
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

#[derive(Debug)]
struct LoopInfo {
    tiles_in_loop: HashSet<(usize, usize)>,
    is_clockwise: bool,
}

fn try_walk(
    grid: &Vec<Vec<u8>>,
    mut current_tile: (usize, usize),
    mut came_from: (i8, i8),
) -> Option<LoopInfo> {
    let mut clockwise_turns = 0;
    let mut anti_clockwise_turns = 0;
    let mut tiles_in_loop = HashSet::new();
    loop {
        tiles_in_loop.insert(current_tile);
        match get_linked_tiles(grid[current_tile.0][current_tile.1]) {
            ConnectedToTiles::Pipe(x) => {
                if !x.contains(&came_from) {
                    break None;
                } else {
                    let out_direction = if x[0] == came_from { x[1] } else { x[0] };

                    match get_turn_type(came_from, out_direction) {
                        TurnType::Straight => {}
                        TurnType::Clockwise => {
                            clockwise_turns += 1;
                        }
                        TurnType::AntiClockwise => {
                            anti_clockwise_turns += 1;
                        }
                    }

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
            ConnectedToTiles::StartTile => {
                break Some(LoopInfo {
                    tiles_in_loop,
                    is_clockwise: clockwise_turns > anti_clockwise_turns,
                })
            }
        }
    }
}

fn count_all_enclosed_tiles(
    grid: &Vec<Vec<u8>>,
    mut current_tile: (usize, usize),
    mut came_from: (i8, i8),
    loop_info: LoopInfo,
) -> usize {
    let max_height = grid.len();
    let max_width = grid[0].len();
    let mut enclosed_tiles = HashSet::new();
    while let ConnectedToTiles::Pipe(x) = get_linked_tiles(grid[current_tile.0][current_tile.1]) {
        let out_direction = if x[0] == came_from { x[1] } else { x[0] };
        let adjacent_tile_directions: Vec<(i8, i8)> =
            match (loop_info.is_clockwise, came_from, out_direction) {
                // | coming from above, left side is enclosed
                (true, (-1, 0), (1, 0)) => vec![(0, -1)],
                // | coming from below, right side is enclosed
                (true, (1, 0), (-1, 0)) => vec![(0, 1)],
                // | coming from above, right side is enclosed
                (false, (-1, 0), (1, 0)) => vec![(0, 1)],
                // | coming from below, left side is enclosed
                (false, (1, 0), (-1, 0)) => vec![(0, -1)],

                // - coming from the left, bottom is enclosed
                (true, (0, -1), (0, 1)) => vec![(1, 0)],
                // - coming from the right, top is enclosed
                (true, (0, 1), (0, -1)) => vec![(-1, 0)],
                // - coming from the left, top is enclosed
                (false, (0, -1), (0, 1)) => vec![(-1, 0)],
                // - coming from the right, bottom is enclosed
                (false, (0, 1), (0, -1)) => vec![(1, 0)],

                // L coming from above, bottom and left side are enclosed
                (true, (-1, 0), (0, 1)) => vec![(1, 0), (0, -1)],
                // L coming from right, no adjacent enclosed tiles
                (true, (0, 1), (-1, 0)) => vec![],
                // L coming from above, no adjacent enclosed tiles
                (false, (-1, 0), (0, 1)) => vec![],
                // L coming from right, bottom and left side are enclosed
                (false, (0, 1), (-1, 0)) => vec![(1, 0), (0, -1)],

                // J coming from above, no adjacent enclosed tiles
                (true, (-1, 0), (0, -1)) => vec![],
                // J coming from left, bottom and right side are enclosed
                (true, (0, -1), (-1, 0)) => vec![(1, 0), (0, 1)],
                // J coming from above, bottom and right side are enclosed
                (false, (-1, 0), (0, -1)) => vec![(1, 0), (0, 1)],
                // J coming from left, no adjacent enclosed tiles
                (false, (0, -1), (-1, 0)) => vec![],

                // 7 coming from below, top and right side are enclosed
                (true, (1, 0), (0, -1)) => vec![(-1, 0), (0, 1)],
                // 7 coming from left, no adjacent enclosed tiles
                (true, (0, -1), (1, 0)) => vec![],
                // 7 coming from below, no adjacent enclosed tiles
                (false, (1, 0), (0, -1)) => vec![],
                // 7 coming from left, top and right side are enclosed
                (false, (0, -1), (1, 0)) => vec![(-1, 0), (0, 1)],

                // F coming from below, no adjacent enclosed tiles
                (true, (1, 0), (0, 1)) => vec![],
                // F coming from right, top and left side are enclosed
                (true, (0, 1), (1, 0)) => vec![(-1, 0), (0, -1)],
                // F coming from below, top and left side are enclosed
                (false, (1, 0), (0, 1)) => vec![(-1, 0), (0, -1)],
                // F coming from right, no adjacent enclosed tiles
                (false, (0, 1), (1, 0)) => vec![],

                _ => panic!("invalid came_from, out_direction pair"),
            };

        for direction in adjacent_tile_directions {
            if let Some(tile) = apply_direction(current_tile, direction, max_height, max_width) {
                walk_enclosed_tiles(
                    tile,
                    max_height,
                    max_width,
                    &mut enclosed_tiles,
                    &loop_info.tiles_in_loop,
                );
            }
        }

        current_tile = apply_direction(current_tile, out_direction, max_height, max_width)
            .expect("should always be a valid direction");
        came_from = invert_direction(out_direction);
    }
    enclosed_tiles.len()
}

fn walk_enclosed_tiles(
    current_tile: (usize, usize),
    max_height: usize,
    max_width: usize,
    enclosed_tiles: &mut HashSet<(usize, usize)>,
    tiles_in_loop: &HashSet<(usize, usize)>,
) {
    let mut tiles_to_check = vec![current_tile];
    while let Some(tile) = tiles_to_check.pop() {
        if !tiles_in_loop.contains(&tile) && !enclosed_tiles.contains(&tile) {
            enclosed_tiles.insert(tile);
            let adjacent_tiles = [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .into_iter()
                .filter_map(|direction| apply_direction(tile, direction, max_height, max_width));
            tiles_to_check.extend(adjacent_tiles);
        }
    }
}

fn solve(str: &str) -> usize {
    let grid = construct_grid(str);
    let starting_tile = find_starting_location(&grid).expect("must have a starting location");
    let (direction, loop_info) = [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .into_iter()
        .find_map(|direction| {
            apply_direction(starting_tile, direction, grid.len(), grid[0].len())
                .and_then(|tile| try_walk(&grid, tile, invert_direction(direction)))
                .map(|loop_info| (direction, loop_info))
        })
        .expect("must be at least one loop");
    count_all_enclosed_tiles(
        &grid,
        apply_direction(starting_tile, direction, grid.len(), grid[0].len())
            .expect("already checked it works above"),
        invert_direction(direction),
        loop_info,
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

        let res = solve(input);

        assert_eq!(res, 4);
    }
    #[test]
    fn example2() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJIF7FJ-
L---JF-JLJIIIIFJLJJ7
|F|F-JF---7IIIL7L|7|
|FFJF7L7F-JF7IIL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

        let res = solve(input);

        assert_eq!(res, 10);
    }
}
