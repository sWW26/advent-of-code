use std::cmp::Ordering;

fn main() {
    let input = include_str!("input1.txt");

    let answer = solve(input);

    println!("{}", answer);
}

#[derive(Debug, Eq, Ord, PartialOrd, PartialEq)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl TryFrom<char> for Card {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Card::Ace),
            'K' => Ok(Card::King),
            'Q' => Ok(Card::Queen),
            'J' => Ok(Card::Jack),
            'T' => Ok(Card::Ten),
            '9' => Ok(Card::Nine),
            '8' => Ok(Card::Eight),
            '7' => Ok(Card::Seven),
            '6' => Ok(Card::Six),
            '5' => Ok(Card::Five),
            '4' => Ok(Card::Four),
            '3' => Ok(Card::Three),
            '2' => Ok(Card::Two),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Eq, Ord, PartialOrd, PartialEq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    hand_type: HandType,
    cards: [Card; 5],
    bid: u32,
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match self.hand_type.cmp(&other.hand_type) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                let mut order = Ordering::Equal;
                for (a, b) in self.cards.iter().zip(&other.cards) {
                    match a.cmp(b) {
                        Ordering::Less => {
                            order = Ordering::Less;
                            break;
                        },
                        Ordering::Equal => {},
                        Ordering::Greater => {
                            order = Ordering::Greater;
                            break;
                        },
                    }
                }
                order
            },
            Ordering::Greater => Ordering::Greater,
        })
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

impl Hand {
    fn new(cards: [Card; 5], bid: u32) -> Self {
        let mut counts = [0_u8; 13];
        for card in &cards {
            match card {
                Card::Ace => counts[0] += 1,
                Card::King => counts[1] += 1,
                Card::Queen => counts[2] += 1,
                Card::Jack => counts[3] += 1,
                Card::Ten => counts[4] += 1,
                Card::Nine => counts[5] += 1,
                Card::Eight => counts[6] += 1,
                Card::Seven => counts[7] += 1,
                Card::Six => counts[8] += 1,
                Card::Five => counts[9] += 1,
                Card::Four => counts[10] += 1,
                Card::Three => counts[11] += 1,
                Card::Two => counts[12] += 1,
            }
        }

        counts.sort_by(|a, b| b.cmp(a));

        let hand_type = match counts {
            [5, ..] => HandType::FiveOfAKind,
            [4, ..] => HandType::FourOfAKind,
            [3, 2, ..] => HandType::FullHouse,
            [3, ..] => HandType::ThreeOfAKind,
            [2, 2, ..] => HandType::TwoPair,
            [2, ..] => HandType::OnePair,
            _ => HandType::HighCard,
        };

        Self {
            cards,
            bid,
            hand_type,
        }
    }
}

fn solve(str: &str) -> u32 {
    let mut hands = str
        .lines()
        .map(|line| {
            let (cards, bid) = line
                .split_once(' ')
                .expect("each line to be in format 'HANDX BID'");
            let cards = cards
                .chars()
                .map(|c| c.try_into().expect("unexpected card"))
                .collect::<Vec<_>>()
                .try_into()
                .expect("should have exactly five cards");
            Hand::new(
                cards,
                bid.parse::<u32>().expect("bid to be a positive number"),
            )
        })
        .collect::<Vec<_>>();
    hands.sort_by(|a, b| b.cmp(a));
    hands
        .into_iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + ((i as u32 + 1) * hand.bid))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let answer = solve(input);

        assert_eq!(answer, 6440);
    }
}
