fn main() {
    let input = include_str!("input1.txt");

    let answer = solve(input);

    println!("{}", answer);
}

fn solve(input: &str) -> u32 {
    let max_by_colour = [("red", 12_u32), ("green", 13), ("blue", 14)];
    input
        .lines()
        .filter_map(|line| {
            let (game_num, rest) = line["Game ".len()..]
                .split_once(':')
                .expect("string should always have ':' after game number");
            let game_num = game_num
                .parse::<u32>()
                .expect("game number must be a number");

            let has_invalid_game = rest.split(';').any(|draw| {
                draw.split(',').any(|colour_count| {
                    let (count, colour) = colour_count
                        .trim()
                        .split_once(' ')
                        .expect("colour count must always be in the format 'N colour'");
                    let (_, max) = max_by_colour
                        .iter()
                        .find(|(c, _)| *c == colour)
                        .expect("colour must be red, green or blue");
                    let count = count.parse::<u32>().expect("colour count must be a number");
                    count > *max
                })
            });
            if has_invalid_game {
                None
            } else {
                Some(game_num)
            }
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

        assert_eq!(answer, 8);
    }
}
