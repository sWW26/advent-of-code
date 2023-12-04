use std::collections::VecDeque;
use std::iter;

fn main() {
    let input = include_str!("input2.txt");

    let answer = solve(input);

    println!("{}", answer);
}

fn calculate_matches(card: &str) -> usize {
    let (winning, have) = card
        .split_once(':')
        .expect("card should always start with 'Card N:'")
        .1
        .split_once('|')
        .expect("card should always have two halves separated by '|'");
    let winning_numbers = winning
        .split(' ')
        .filter(|x| !x.is_empty())
        .map(|x| {
            x.parse::<u32>()
                .expect("all winning components should be numbers")
        })
        .collect::<Vec<_>>();
    let matching_count = have
        .split(' ')
        .filter(|x| !x.is_empty())
        .map(|x| {
            x.parse::<u32>()
                .expect("all have components should be numbers")
        })
        .filter(|n| winning_numbers.contains(&n))
        .count();

    matching_count
}

fn solve(str: &str) -> u32 {
    let mut total_cards = 0;
    let mut additional_copies = VecDeque::new();
    for line in str.lines() {
        let copies = additional_copies.pop_front().unwrap_or(0) + 1;

        let matches = calculate_matches(line);
        if matches > 0 {
            let mut x = iter::repeat(copies).take(matches);

            for c in &mut additional_copies {
                if let Some(copies) = x.next() {
                    *c += copies;
                }
            }

            additional_copies.extend(x);
        }

        total_cards += copies;
    }

    total_cards
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let answer = solve(input);

        assert_eq!(answer, 30);
    }
}
