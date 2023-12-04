use std::ops::RangeInclusive;
use std::str::Lines;

fn main() {
    let input = include_str!("input2.txt");

    let answer = solve(input);

    println!("{}", answer);
}

#[derive(Debug, PartialEq)]
struct WindowedTriplet<'a> {
    previous: Option<&'a str>,
    current: &'a str,
    next: Option<&'a str>,
}

fn get_windowed_triplet<'a>(mut lines: Lines<'a>) -> impl Iterator<Item = WindowedTriplet<'a>> {
    let mut previous = None;
    let mut current = lines.next();
    std::iter::from_fn(move || {
        current.map(|c| {
            let next = lines.next();
            let triplet = WindowedTriplet {
                previous,
                current: c,
                next,
            };
            previous = Some(c);
            current = next;
            triplet
        })
    })
}

fn find_full_number_range(line: &str, first_known_position: usize) -> RangeInclusive<usize> {
    let str_bytes = line.as_bytes();
    let mut start = first_known_position;
    let mut end = first_known_position;
    while start > 0 && str_bytes[start - 1].is_ascii_digit() {
        start -= 1;
    }
    while end < str_bytes.len() - 1 && str_bytes[end + 1].is_ascii_digit() {
        end += 1;
    }

    start..=end
}

fn check_adjacent_line(line: &str, star_position: usize) -> Vec<u32> {
    let str_bytes = line.as_bytes();
    let mut numbers = Vec::new();
    let mut i = if star_position > 0 {
        star_position - 1
    } else {
        0
    };
    while i <= std::cmp::min(star_position + 1, line.len() - 1) {
        if str_bytes[i].is_ascii_digit() {
            let range = find_full_number_range(line, i);
            i = range.end() + 1;
            let num = line[range]
                .parse::<u32>()
                .expect("all characters to be digits");
            numbers.push(num);
        } else {
            i += 1;
        }
    }

    numbers
}

fn get_adjacent_numbers(lines: &WindowedTriplet, star_position: usize) -> Vec<u32> {
    let mut numbers = Vec::with_capacity(2);
    if star_position > 0 && lines.current.as_bytes()[star_position - 1].is_ascii_digit() {
        let range = find_full_number_range(lines.current, star_position - 1);
        let num = lines.current[range]
            .parse::<u32>()
            .expect("all characters to be digits");
        numbers.push(num);
    }

    if star_position < lines.current.len() - 1
        && lines.current.as_bytes()[star_position + 1].is_ascii_digit()
    {
        let range = find_full_number_range(lines.current, star_position + 1);
        let num = lines.current[range]
            .parse::<u32>()
            .expect("all characters to be digits");
        numbers.push(num);
    }

    if let Some(prev) = lines.previous {
        let nums = check_adjacent_line(prev, star_position);
        numbers.extend(nums);
    }

    if let Some(next) = lines.next {
        let nums = check_adjacent_line(next, star_position);
        numbers.extend(nums);
    }

    numbers
}

fn solve(str: &str) -> u32 {
    let lines = str.lines();
    let windowed_triplets = get_windowed_triplet(lines);
    windowed_triplets
        .flat_map(|triplet| {
            triplet
                .current
                .bytes()
                .enumerate()
                .filter_map(|(i, byte)| {
                    if byte == ('*' as u8) {
                        let numbers = get_adjacent_numbers(&triplet, i);
                        if numbers.len() > 1 {
                            Some(numbers.into_iter().reduce(|a, b| a * b).unwrap())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let answer = solve(input);

        assert_eq!(answer, 467835);
    }
}
