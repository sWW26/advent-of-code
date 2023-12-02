use std::cmp::max;

fn main() {
    let input = include_str!("input2.txt");

    let answer = solve(input);

    println!("{}", answer);
}

struct Rgb {
    r: u32,
    g: u32,
    b: u32,
}

fn solve(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (_, rest) = line
                .split_once(':')
                .expect("string should always have ':' after game number");

            let min_rgb_required =
                rest.split(';').fold(Rgb { r: 0, g: 0, b: 0 }, |mut acc: Rgb, draw| {
                    draw.split(',').for_each(|colour_count| {
                        let (count, colour) = colour_count
                            .trim()
                            .split_once(' ')
                            .expect("colour count must always be in the format 'N colour'");
                        let count = count.parse::<u32>().expect("colour count must be a number");
                        match colour {
                            "red" => acc.r = max(count, acc.r),
                            "green" => acc.g = max(count, acc.g),
                            "blue" => acc.b = max(count, acc.b),
                            _ => panic!("invalid colour {}", colour)
                        };
                    });
                    acc
                });

            min_rgb_required.r * min_rgb_required.g * min_rgb_required.b
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let answer = solve(&input);

        assert_eq!(answer, 2286);
    }
}
