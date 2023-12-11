use std::collections::HashMap;

fn main() {
    let input = include_str!("input1.txt");

    let answer = solve(input);

    println!("{}", answer);
}

fn solve(str: &str) -> u32 {
    let mut lines = str.lines();
    let mut sequence = lines
        .next()
        .expect("sequence on first line")
        .bytes()
        .cycle()
        .enumerate();
    let _ = lines.next().expect("blank line");
    let mut map = HashMap::new();
    for items in lines {
        let key: [u8; 3] = items.as_bytes()[..3].try_into().unwrap();
        let left: [u8; 3] = items.as_bytes()[7..10].try_into().unwrap();
        let right: [u8; 3] = items.as_bytes()[12..15].try_into().unwrap();
        map.insert(key, (left, right));
    }
    let end: [u8; 3] = "ZZZ".as_bytes().try_into().unwrap();
    let mut current_node: [u8; 3] = "AAA".as_bytes().try_into().unwrap();
    let mut steps = 0;
    while current_node != end {
        let (i, dir) = sequence.next().expect("sequence should be infinite");
        let (l, r) = map.get(&current_node).expect("node must be in the map");
        current_node = if dir == b'L' { *l } else { *r };
        steps = i;
    }
    steps as u32 + 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        let answer = solve(input);

        assert_eq!(answer, 2);
    }

    #[test]
    fn example_two() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        let answer = solve(input);

        assert_eq!(answer, 6);
    }
}
