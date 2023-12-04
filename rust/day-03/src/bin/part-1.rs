use std::str::Lines;

fn main() {
    let input = include_str!("input1.txt");

    let answer = solve(input);

    println!("{}", answer);
}

#[derive(Debug, PartialEq)]
struct WindowedTriplet<'a> {
    previous: Option<&'a str>,
    current: &'a str,
    next: Option<&'a str>,
}

fn get_windowed_triplet<'a>(mut lines: Lines<'a>) -> impl Iterator<Item=WindowedTriplet<'a>> {
    let mut previous = None;
    let mut current = lines.next();
    std::iter::from_fn(move || {
        current.map(|c| {
            let next = lines.next();
            let triplet = WindowedTriplet {
                previous: previous,
                current: c,
                next: next,
            };
            previous = Some(c);
            current = next;
            triplet
        })
    })
}

fn char_is_symbol(c: u8) -> bool {
    !c.is_ascii_digit() && c != ('.' as u8)
}

fn number_touches_symbol(triplet: &WindowedTriplet, start: usize, end: usize) -> bool {
    let prev_char_same_line_is_symbol =
        || start > 0 && char_is_symbol(triplet.current.as_bytes()[start - 1]);

    let next_char_same_line_is_symbol =
        || triplet
            .current
            .as_bytes()
            .get(end + 1)
            .map(|c| char_is_symbol(*c))
            .unwrap_or(false);

    let line_adjacent_contains_symbol =
        |line: Option<&str>| {
            let start = if start == 0 { 0 } else { start - 1 };
            line
                .map(|p| {
                    let end = if end + 1 >= p.len() {
                        p.len() - 1
                    } else {
                        end + 1
                    };
                    p[start..=end]
                        .bytes()
                        .any(char_is_symbol)
                })
                .unwrap_or(false)
        };

    prev_char_same_line_is_symbol()
        || next_char_same_line_is_symbol()
        || line_adjacent_contains_symbol(triplet.previous)
        || line_adjacent_contains_symbol(triplet.next)
}

fn solve(str: &str) -> u32 {
    let lines = str.lines();
    let windowed_triplets = get_windowed_triplet(lines);
    windowed_triplets
        .flat_map(|triplet| {
            let (start, number_string, mut numbers) = triplet.current.bytes().enumerate().fold(
                (
                    None,
                    String::with_capacity(triplet.current.len()),
                    Vec::new(),
                ),
                |(start_index, mut number_string, mut numbers), (i, byte)| {
                    let new_range = if byte.is_ascii_digit() {
                        number_string.push(byte as char);
                        start_index.or_else(|| Some(i))
                    } else {
                        if let Some(start) = start_index {
                            let number = number_string.parse::<u32>().unwrap();
                            if number_touches_symbol(&triplet, start, start + number_string.len() - 1) {
                                numbers.push(number);
                            }
                        }
                        number_string.clear();
                        None
                    };
                    (new_range, number_string, numbers)
                },
            );

            if let Some(start) = start {
                let number = number_string.parse::<u32>().unwrap();
                if number_touches_symbol(&triplet, start, start + number_string.len() - 1) {
                    numbers.push(number);
                }
            }

            numbers
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

        assert_eq!(answer, 4361);
    }

    mod windowed_triplet {
        use super::*;

        #[test]
        fn empty_returns_none() {
            let input = "".lines();
            let res = get_windowed_triplet(input).collect::<Vec<_>>();
            assert_eq!(res, vec![]);
        }

        #[test]
        fn one_line_returns_one() {
            let input = "a".lines();
            let res = get_windowed_triplet(input).collect::<Vec<_>>();
            assert_eq!(
                res,
                vec![WindowedTriplet {
                    previous: None,
                    current: "a",
                    next: None,
                }]
            );
        }

        #[test]
        fn two_lines_returns_two() {
            let input = "a
b"
                .lines();
            let res = get_windowed_triplet(input).collect::<Vec<_>>();
            assert_eq!(
                res,
                vec![
                    WindowedTriplet {
                        previous: None,
                        current: "a",
                        next: Some("b"),
                    },
                    WindowedTriplet {
                        previous: Some("a"),
                        current: "b",
                        next: None,
                    },
                ]
            );
        }

        #[test]
        fn three_lines_returns_three() {
            let input = "a
b
c"
                .lines();
            let res = get_windowed_triplet(input).collect::<Vec<_>>();
            assert_eq!(
                res,
                vec![
                    WindowedTriplet {
                        previous: None,
                        current: "a",
                        next: Some("b"),
                    },
                    WindowedTriplet {
                        previous: Some("a"),
                        current: "b",
                        next: Some("c"),
                    },
                    WindowedTriplet {
                        previous: Some("b"),
                        current: "c",
                        next: None,
                    },
                ]
            );
        }
    }

    mod number_touches_symbol {
        use super::*;

        #[test]
        fn first_line() {
            let input = WindowedTriplet {
                previous: None,
                current: "467..114..",
                next: Some("...*......"),
            };

            let res = number_touches_symbol(&input, 0, 2);

            assert!(res);
        }

        #[test]
        fn number_at_end_of_line() {
            let input = WindowedTriplet {
                previous: Some(".............24............*............*........601.346...621......531.....452......*.....659.......-297..697..468................*.936...."),
                current: "........265............905..64.&...589..960..................*.......*..........62..19.........152................*.......+.....243......962",
                next: Some("...........*..220......*.......757.#..............-270..697..588....461..263......*.......@373....*........464...244...688..............*..."),
            };

            let res = number_touches_symbol(&input, 137, 139);

            assert!(res);
        }
    }
}
